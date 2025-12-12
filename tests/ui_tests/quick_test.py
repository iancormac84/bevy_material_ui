"""
Bevy Material UI - Quick Test Script
====================================

Runs targeted tests on specific components and reports findings for AI iteration.
Now with telemetry support - reads component state from telemetry.json
Includes visual regression testing via screenshot comparison.
"""

import subprocess
import time
import sys
import json
import argparse
from pathlib import Path
from datetime import datetime

try:
    import pyautogui
    from PIL import ImageGrab
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "pyautogui", "pillow", "numpy"], check=True)
    import pyautogui
    from PIL import ImageGrab

# Import our visual diff module
from visual_diff import compare_with_baseline, generate_report, save_baseline

pyautogui.FAILSAFE = False
pyautogui.PAUSE = 0.1

OUTPUT_DIR = Path(__file__).parent / "test_output"
OUTPUT_DIR.mkdir(exist_ok=True)
WORKSPACE_DIR = Path(__file__).parent.parent.parent
TELEMETRY_FILE = WORKSPACE_DIR / "telemetry.json"

# Track visual regression results
visual_results = []

# Track active window bounds for click validation
_window_bounds = None  # (left, top, right, bottom) of the application window
_client_origin = None  # (x, y) of the client area origin


def set_window_bounds(window_rect, client_origin):
    """Set the window bounds for click validation"""
    global _window_bounds, _client_origin
    _window_bounds = window_rect
    _client_origin = client_origin
    if window_rect:
        print(f"  Click bounds set: ({window_rect[0]}, {window_rect[1]}) to ({window_rect[2]}, {window_rect[3]})")


def is_click_in_bounds(x: float, y: float) -> tuple[bool, str]:
    """Check if a click coordinate is within the application window bounds.
    Returns (is_valid, reason_if_invalid)"""
    if _window_bounds is None:
        return True, ""  # No bounds set, allow click
    
    left, top, right, bottom = _window_bounds
    
    # Add small margin (5px) to account for window decorations
    margin = 5
    
    if x < left + margin:
        return False, f"x={x:.0f} is left of window (left bound: {left})"
    if x > right - margin:
        return False, f"x={x:.0f} is right of window (right bound: {right})"
    if y < top + margin:
        return False, f"y={y:.0f} is above window (top bound: {top})"
    if y > bottom - margin:
        return False, f"y={y:.0f} is below window (bottom bound: {bottom})"
    
    return True, ""


def safe_click(x: float, y: float, description: str = "") -> bool:
    """Perform a click only if coordinates are within window bounds.
    Returns True if click was performed, False if blocked."""
    is_valid, reason = is_click_in_bounds(x, y)
    
    if not is_valid:
        print(f"  [BLOCKED] Click at ({x:.0f}, {y:.0f}) would be outside application: {reason}")
        if description:
            print(f"            Attempted action: {description}")
        return False
    
    pyautogui.moveTo(x, y, duration=0.1)
    time.sleep(0.05)
    pyautogui.click()
    return True


def read_telemetry(max_retries: int = 3):
    """Read component telemetry from the app with retry logic for race conditions"""
    if TELEMETRY_FILE.exists():
        for attempt in range(max_retries):
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip():  # Ensure file isn't empty
                        return json.loads(content)
                    # File was empty, wait and retry
                    time.sleep(0.05)
            except json.JSONDecodeError:
                # File was being written to, wait and retry
                time.sleep(0.05)
            except Exception as e:
                print(f"Error reading telemetry: {e}")
                return None
    return None


def wait_for_telemetry_state(key: str, expected_value: str, timeout: float = 2.0) -> bool:
    """Wait until telemetry shows expected state"""
    start = time.time()
    while time.time() - start < timeout:
        telemetry = read_telemetry()
        if telemetry and telemetry.get("states", {}).get(key) == expected_value:
            return True
        time.sleep(0.1)
    return False


def verify_telemetry_state(key: str, expected_value: str, wait_first: bool = True) -> dict:
    """Verify a telemetry state matches expected value, with optional wait"""
    if wait_first:
        # Wait briefly for the state to settle (up to 1 second)
        wait_for_telemetry_state(key, expected_value, timeout=1.0)
    
    telemetry = read_telemetry()
    actual = telemetry.get("states", {}).get(key) if telemetry else None
    passed = actual == expected_value
    return {
        "key": key,
        "expected": expected_value,
        "actual": actual,
        "passed": passed,
        "message": f"{'[PASS]' if passed else '[FAIL]'} {key}: expected '{expected_value}', got '{actual}'"
    }


def get_element_bounds(test_id: str) -> dict:
    """Get element bounds by test_id from telemetry"""
    telemetry = read_telemetry()
    if not telemetry:
        return None
    elements = telemetry.get("elements", [])
    for elem in elements:
        if elem.get("test_id") == test_id:
            return elem
    return None


def click_element(test_id: str, window_rect=None) -> bool:
    """Click an element by its test_id using bounds from telemetry.
    Uses safe_click to prevent clicking outside the application."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    # Get window offset - prefer client origin if available, fallback to window_rect
    if _client_origin:
        win_x, win_y = _client_origin
    elif window_rect:
        win_x, win_y = window_rect[0], window_rect[1]
    else:
        win_x, win_y = 0, 0
    
    # Calculate center of element (bounds are already in screen coordinates)
    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2
    
    print(f"  Clicking '{test_id}' at ({center_x:.0f}, {center_y:.0f}) [bounds: x={bounds['x']:.0f}, y={bounds['y']:.0f}, w={bounds['width']:.0f}, h={bounds['height']:.0f}]")
    
    # Use safe_click to prevent clicking outside window
    if not safe_click(center_x, center_y, f"click element '{test_id}'"):
        return False
    
    time.sleep(0.3)
    return True


def drag_element(test_id: str, delta_x: float, delta_y: float, window_rect=None, duration=0.3) -> bool:
    """Drag an element by its test_id with given delta.
    Validates both start and end positions are within window bounds."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    # Use client origin if available, fallback to window_rect
    if _client_origin:
        win_x, win_y = _client_origin
    elif window_rect:
        win_x, win_y = window_rect[0], window_rect[1]
    else:
        win_x, win_y = 0, 0
    
    start_x = win_x + bounds["x"] + bounds["width"] / 2
    start_y = win_y + bounds["y"] + bounds["height"] / 2
    end_x = start_x + delta_x
    end_y = start_y + delta_y
    
    # Validate both start and end are in bounds
    start_valid, start_reason = is_click_in_bounds(start_x, start_y)
    end_valid, end_reason = is_click_in_bounds(end_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Drag start at ({start_x:.0f}, {start_y:.0f}) outside application: {start_reason}\")")
        return False
    if not end_valid:
        print(f"  [BLOCKED] Drag end at ({end_x:.0f}, {end_y:.0f}) would be outside application: {end_reason}\")")
        return False
    
    print(f"  Dragging '{test_id}' from ({start_x:.0f}, {start_y:.0f}) by ({delta_x:.0f}, {delta_y:.0f})")
    pyautogui.moveTo(start_x, start_y)
    pyautogui.drag(delta_x, delta_y, duration=duration)
    time.sleep(0.3)
    return True


def drag_scrollbar(scrollbar_id: str, amount: float, client_origin=None) -> bool:
    """Drag a scrollbar thumb by a given amount (positive = down, negative = up).
    
    Args:
        scrollbar_id: TestId of the scrollbar thumb (e.g., 'sidebar_scroll_thumb', 'main_scroll_thumb')
        amount: Pixels to drag (positive = scroll down, negative = scroll up)
        client_origin: Client origin from find_bevy_window
    
    Returns:
        True if drag was performed, False if blocked or element not found
    """
    bounds = get_element_bounds(scrollbar_id)
    if not bounds:
        print(f"  [MISS] Scrollbar '{scrollbar_id}' not found in telemetry")
        return False
    
    # Use client origin if available
    if client_origin and _client_origin:
        win_x, win_y = _client_origin
    elif client_origin:
        win_x, win_y = client_origin
    else:
        win_x, win_y = 0, 0
    
    # Calculate center of scrollbar thumb
    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2
    
    # Calculate end position
    end_y = center_y + amount
    
    # Validate both positions are in bounds
    start_valid, start_reason = is_click_in_bounds(center_x, center_y)
    end_valid, end_reason = is_click_in_bounds(center_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Scrollbar at ({center_x:.0f}, {center_y:.0f}) outside window: {start_reason}")
        return False
    if not end_valid:
        # Clamp to window bounds instead of blocking
        if _window_bounds:
            end_y = max(_window_bounds[1] + 10, min(_window_bounds[3] - 10, end_y))
            print(f"  [CLAMPED] Scrollbar drag end clamped to ({center_x:.0f}, {end_y:.0f})")
    
    print(f"  Dragging scrollbar '{scrollbar_id}' from y={center_y:.0f} to y={end_y:.0f} (delta={amount:.0f})")
    
    pyautogui.moveTo(center_x, center_y)
    time.sleep(0.1)
    pyautogui.mouseDown()
    time.sleep(0.05)
    pyautogui.moveTo(center_x, end_y, duration=0.3)
    pyautogui.mouseUp()
    time.sleep(0.3)
    
    return True


def get_scroll_position(scroll_key: str = "sidebar_scroll_y") -> float:
    """Get current scroll position from telemetry.
    
    Args:
        scroll_key: Telemetry key - 'sidebar_scroll_y' or 'main_scroll_y'
    
    Returns:
        Scroll position in pixels, or 0.0 if not found
    """
    telemetry = read_telemetry()
    if telemetry:
        value = telemetry.get("states", {}).get(scroll_key, "0.0")
        try:
            return float(value)
        except (ValueError, TypeError):
            return 0.0
    return 0.0


def list_available_elements():
    """Print all available elements with test_ids"""
    telemetry = read_telemetry()
    if not telemetry:
        print("No telemetry available")
        return
    elements = telemetry.get("elements", [])
    print(f"\nAvailable elements ({len(elements)} total):")
    for elem in sorted(elements, key=lambda e: e.get("test_id", "")):
        print(f"  - {elem.get('test_id')}: ({elem.get('x'):.0f}, {elem.get('y'):.0f}) {elem.get('width'):.0f}x{elem.get('height'):.0f}")


def find_bevy_window(maximize: bool = True):
    """Find, focus, and optionally maximize the Bevy window. Returns (client_origin, window_rect)"""
    try:
        import ctypes
        import ctypes.wintypes
        user32 = ctypes.windll.user32
        
        # Constants for ShowWindow
        SW_MAXIMIZE = 3
        SW_RESTORE = 9
        SW_SHOWNORMAL = 1
        
        # Get all window handles
        def get_windows():
            windows = []
            def callback(hwnd, _):
                if user32.IsWindowVisible(hwnd):
                    length = user32.GetWindowTextLengthW(hwnd) + 1
                    buffer = ctypes.create_unicode_buffer(length)
                    user32.GetWindowTextW(hwnd, buffer, length)
                    if buffer.value:
                        windows.append((hwnd, buffer.value))
                return True
            
            WNDENUMPROC = ctypes.WINFUNCTYPE(ctypes.c_bool, ctypes.c_int, ctypes.c_int)
            user32.EnumWindows(WNDENUMPROC(callback), 0)
            return windows
        
        windows = get_windows()
        for hwnd, title in windows:
            if "Material Design 3" in title or "showcase" in title.lower() or "bevy app" in title.lower():
                print(f"Found Bevy window: {title}")
                
                # Restore if minimized
                user32.ShowWindow(hwnd, SW_RESTORE)
                time.sleep(0.2)
                
                # Maximize or use normal size
                if maximize:
                    user32.ShowWindow(hwnd, SW_MAXIMIZE)
                    print("  Window: MAXIMIZED")
                else:
                    user32.ShowWindow(hwnd, SW_SHOWNORMAL)
                    print("  Window: NORMAL")
                time.sleep(0.3)
                
                # Focus window
                user32.SetForegroundWindow(hwnd)
                time.sleep(0.3)
                
                # Get window rect
                class RECT(ctypes.Structure):
                    _fields_ = [("left", ctypes.c_long), ("top", ctypes.c_long),
                               ("right", ctypes.c_long), ("bottom", ctypes.c_long)]
                rect = RECT()
                user32.GetWindowRect(hwnd, ctypes.byref(rect))
                
                # Get client area position for accurate clicking
                client_point = ctypes.wintypes.POINT(0, 0)
                user32.ClientToScreen(hwnd, ctypes.byref(client_point))
                
                window_rect = (rect.left, rect.top, rect.right, rect.bottom)
                client_origin = (client_point.x, client_point.y)
                
                print(f"  Window rect: {window_rect}")
                print(f"  Client origin (for element clicks): {client_origin}")
                
                # Set global window bounds for click validation
                set_window_bounds(window_rect, client_origin)
                
                return client_origin, window_rect
    except Exception as e:
        print(f"Window detection error: {e}")
    
    return None, None


def capture(name: str, rect=None, check_baseline: bool = False):
    """Capture screenshot, optionally comparing to baseline"""
    path = OUTPUT_DIR / f"{name}_{datetime.now().strftime('%H%M%S')}.png"
    if rect:
        img = ImageGrab.grab(bbox=rect)
    else:
        img = ImageGrab.grab()
    img.save(path)
    
    # Visual regression check - use higher threshold since window position varies
    if check_baseline:
        result = compare_with_baseline(name, img, threshold=0.40)  # 40% threshold for position variance
        visual_results.append(result)
        print(f"  Visual: {result['message']}")
    
    return path


def click_relative(rect, rel_x, rel_y):
    """Click at relative position within window.
    Uses safe_click to prevent clicking outside the application."""
    if not rect:
        return False
    x = rect[0] + int(rel_x * (rect[2] - rect[0]))
    y = rect[1] + int(rel_y * (rect[3] - rect[1]))
    
    if not safe_click(x, y, f"click at relative ({rel_x:.2f}, {rel_y:.2f})"):
        return False
    
    time.sleep(0.3)
    return True


def drag_relative(rect, start, end, duration=0.3):
    """Drag from start to end (relative coords).
    Validates both start and end positions are within window bounds."""
    if not rect:
        return False
    w = rect[2] - rect[0]
    h = rect[3] - rect[1]
    
    start_x = rect[0] + int(start[0] * w)
    start_y = rect[1] + int(start[1] * h)
    end_x = rect[0] + int(end[0] * w)
    end_y = rect[1] + int(end[1] * h)
    
    # Validate both start and end are in bounds
    start_valid, start_reason = is_click_in_bounds(start_x, start_y)
    end_valid, end_reason = is_click_in_bounds(end_x, end_y)
    
    if not start_valid:
        print(f"  [BLOCKED] Drag start at ({start_x}, {start_y}) outside application: {start_reason}\")")
        return False
    if not end_valid:
        print(f"  [BLOCKED] Drag end at ({end_x}, {end_y}) would be outside application: {end_reason}\")")
        return False
    
    pyautogui.moveTo(start_x, start_y)
    pyautogui.drag(end_x - start_x, end_y - start_y, duration=duration)
    time.sleep(0.3)
    return True


def test_sliders(rect, client_origin=None):
    """Slider testing using element-based clicking"""
    print("\n=== SLIDER TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Sliders section using element bounds
    print("\n[Setup] Navigating to Sliders...")
    if click_element("nav_sliders", click_pos):
        time.sleep(0.6)
        result = verify_telemetry_state("selected_section", "Sliders")
        print(f"  {result['message']}")
    
    capture("slider_section", rect, check_baseline=True)
    
    # Test 1: Drag slider thumb using element bounds
    print("\n[Test 1] Continuous Slider Drag")
    if drag_element("slider_thumb_0", 150, 0, click_pos, duration=0.5):
        time.sleep(0.4)
        telemetry = read_telemetry()
        slider_val = telemetry.get("states", {}).get("slider_0_value", "N/A") if telemetry else "N/A"
        print(f"  Slider 0 value: {slider_val}")
        capture("slider_continuous_after", rect, check_baseline=True)
        observations.append({
            "test": "Continuous Slider Drag",
            "action": "Dragged slider_thumb_0 by 150px",
            "value": slider_val,
            "verify": "Check if thumb moved and value display updated"
        })
    else:
        print("  [SKIP] slider_thumb_0 not found")
    
    # Test 2: Drag second slider thumb
    print("\n[Test 2] Discrete Slider Drag")
    if drag_element("slider_thumb_1", 100, 0, click_pos, duration=0.5):
        time.sleep(0.4)
        telemetry = read_telemetry()
        slider_val = telemetry.get("states", {}).get("slider_1_value", "N/A") if telemetry else "N/A"
        print(f"  Slider 1 value: {slider_val}")
        capture("slider_discrete_after", rect, check_baseline=True)
        observations.append({
            "test": "Discrete Slider Drag", 
            "action": "Dragged slider_thumb_1 by 100px",
            "value": slider_val,
            "verify": "Check if thumb snaps to tick positions"
        })
    else:
        print("  [SKIP] slider_thumb_1 not found")
    
    # Test 3: Click on track
    print("\n[Test 3] Track Click")
    if click_element("slider_track_0", click_pos):
        time.sleep(0.3)
        capture("slider_track_after", rect)
        observations.append({
            "test": "Track Click",
            "action": "Clicked directly on slider_track_0",
            "verify": "Slider should respond to track click"
        })
    
    return observations


def test_tabs(rect, client_origin=None):
    """Tab testing using element-based clicking"""
    print("\n=== TAB TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Tabs section using element bounds
    print("\n[Setup] Navigating to Tabs section...")
    if click_element("nav_tabs", click_pos):
        time.sleep(0.6)
        result = verify_telemetry_state("selected_section", "Tabs")
        telemetry_checks.append(result)
        print(f"  {result['message']}")
    else:
        print("  [SKIP] nav_tabs not found - may need to scroll sidebar")
    
    capture("tabs_section", rect, check_baseline=True)
    
    # Test: Verify initial tab state
    print("\n[Test] Verify initial tab state")
    result = verify_telemetry_state("tab_selected", "0")
    telemetry_checks.append(result)
    print(f"  {result['message']}")
    capture("tabs_initial", rect, check_baseline=True)
    
    # Test: Click Tab 2 using element bounds
    print("\n[Test] Select Tab 2")
    if click_element("tab_2", click_pos):
        time.sleep(0.6)  # Slightly longer wait
        result = verify_telemetry_state("tab_selected", "1")
        telemetry_checks.append(result)
        print(f"  {result['message']}")
        capture("tabs_tab2_selected", rect, check_baseline=True)
        observations.append({
            "test": "Tab Selection",
            "action": "Clicked tab_2 using element bounds",
            "telemetry_verified": result["passed"],
            "verify": [
                "Tab 2 text should be primary color",
                "Tab 2 should have 3px bottom border indicator",
                "Tab 2 content panel should be visible"
            ]
        })
    else:
        print("  [SKIP] tab_2 not found in telemetry")
    
    # Test: Click Tab 3
    print("\n[Test] Select Tab 3")
    # Show tab positions for debugging
    tab2_bounds = get_element_bounds("tab_2")
    tab3_bounds = get_element_bounds("tab_3")
    if tab2_bounds:
        print(f"  tab_2 position: x={tab2_bounds['x']}, y={tab2_bounds['y']}, w={tab2_bounds['width']}, h={tab2_bounds['height']}")
    if tab3_bounds:
        print(f"  tab_3 position: x={tab3_bounds['x']}, y={tab3_bounds['y']}, w={tab3_bounds['width']}, h={tab3_bounds['height']}")
    
    if click_element("tab_3", click_pos):
        time.sleep(0.8)  # Longer wait to ensure state settles
        result = verify_telemetry_state("tab_selected", "2")
        telemetry_checks.append(result)
        print(f"  {result['message']}")
        capture("tabs_tab3_selected", rect, check_baseline=True)
    else:
        print("  [SKIP] tab_3 not found in telemetry")
    
    passed = sum(1 for c in telemetry_checks if c["passed"])
    print(f"\n  Telemetry checks: {passed}/{len(telemetry_checks)} passed")
    
    return observations


def test_nav_highlighting(rect, client_origin=None):
    """Test sidebar navigation highlighting using element-based clicking"""
    print("\n=== NAVIGATION HIGHLIGHTING TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    # Use client_origin for element clicking if provided
    click_pos = client_origin if client_origin else rect
    
    # Test nav items that should be visible
    sections = [
        ("nav_buttons", "Buttons"),
        ("nav_checkboxes", "Checkboxes"),
        ("nav_chips", "Chips"),
        ("nav_fab", "FAB"),
    ]
    
    for nav_id, expected_section in sections:
        print(f"\n[Test] Navigate to {expected_section}")
        if click_element(nav_id, click_pos):
            time.sleep(0.8)  # Longer wait for navigation animation
            result = verify_telemetry_state("selected_section", expected_section)
            telemetry_checks.append(result)
            print(f"  {result['message']}")
            capture(f"nav_{expected_section.lower()}", rect, check_baseline=True)
        else:
            print(f"  [SKIP] {nav_id} not found")
        time.sleep(0.3)  # Extra delay between nav clicks
        
    observations.append({
        "test": "Navigation Highlighting",
        "action": "Clicked through multiple sidebar sections using element bounds",
        "telemetry_verified": all(c["passed"] for c in telemetry_checks) if telemetry_checks else False,
        "verify": [
            "Selected item should have secondary_container background color",
            "Other items should have transparent background",
            "Selection should persist until another item is clicked"
        ]
    })
    
    passed = sum(1 for c in telemetry_checks if c["passed"])
    print(f"\n  Telemetry checks: {passed}/{len(telemetry_checks)} passed")
    
    return observations


def test_checkboxes(rect, client_origin=None):
    """Test checkbox toggling using element-based clicking"""
    print("\n=== CHECKBOX TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Navigate to Checkboxes section
    print("\n[Setup] Navigating to Checkboxes...")
    if click_element("nav_checkboxes", click_pos):
        time.sleep(0.6)
        result = verify_telemetry_state("selected_section", "Checkboxes")
        print(f"  {result['message']}")
    
    capture("checkbox_initial", rect, check_baseline=True)
    
    # Toggle first checkbox using its test_id
    print("\n[Test] Toggle checkbox_0")
    if click_element("checkbox_0", click_pos):
        time.sleep(0.4)
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        toggled = any("Checkbox" in e for e in events[-3:])
        print(f"  Checkbox toggled: {toggled}")
        capture("checkbox_toggled", rect, check_baseline=True)
        
        # Toggle again
        print("\n[Test] Toggle checkbox_0 again")
        if click_element("checkbox_0", click_pos):
            time.sleep(0.4)
            capture("checkbox_untoggled", rect, check_baseline=True)
    else:
        print("  [SKIP] checkbox_0 not found in telemetry")
    
    observations.append({
        "test": "Checkbox Toggle",
        "action": "Clicked checkbox using element bounds",
        "verify": [
            "Checkbox should show checkmark icon when checked",
            "Checkbox should be empty when unchecked",
            "Background color should change based on state"
        ]
    })
    
    return observations


def test_menus(rect, client_origin=None):
    """Test menu interactions using element-based clicking"""
    print("\n=== MENU TESTS (Element-Based) ===")
    observations = []
    
    click_pos = client_origin if client_origin else rect
    
    # Scroll sidebar to show nav_menus
    print("\n[Setup] Scrolling sidebar to show Menus...")
    scroll_sidebar_to_top(click_pos)
    time.sleep(0.2)
    # Menus is further down, scroll down a bit
    pyautogui.moveTo(click_pos[0] + 150, click_pos[1] + 400)
    pyautogui.scroll(-5)  # Scroll down
    time.sleep(0.3)
    
    # Navigate to Menus section using element bounds
    print("\n[Setup] Navigating to Menus section...")
    time.sleep(0.5)  # Allow previous section to settle
    if click_element("nav_menus", click_pos):
        time.sleep(1.0)  # Longer wait for menu section to load
        result = verify_telemetry_state("selected_section", "Menus")
        print(f"  {result['message']}")
    else:
        print("  [SKIP] nav_menus not found - may need to scroll sidebar")
    
    capture("menu_section", rect, check_baseline=True)
    
    # Menu interactions still use relative coordinates since menu items
    # may not have TestId yet - but we'll improve this later
    window_width = rect[2] - rect[0]
    window_height = rect[3] - rect[1]
    sidebar_width_px = 220
    header_height_px = 50
    
    content_start_x = sidebar_width_px / window_width
    menu_trigger_x = content_start_x + 0.15
    menu_trigger_y = (header_height_px + 120) / window_height
    
    print("\n[Test] Open Menu")
    print(f"  Clicking trigger at ({menu_trigger_x:.3f}, {menu_trigger_y:.3f})")
    click_relative(rect, menu_trigger_x, menu_trigger_y)
    time.sleep(0.4)
    capture("menu_open", rect, check_baseline=True)
    
    # Click menu item
    menu_item_y = menu_trigger_y + 0.08
    print("\n[Test] Select Menu Item") 
    click_relative(rect, menu_trigger_x, menu_item_y)
    time.sleep(0.3)
    capture("menu_selected", rect, check_baseline=True)
    
    observations.append({
        "test": "Menu Dropdown",
        "action": "Opened menu and selected item",
        "verify": [
            "Menu should appear below trigger",
            "Menu items should be visible and clickable",
            "Menu should close after selection"
        ]
    })
    
    # Test keyboard shortcut
    print("\n[Test] Keyboard Shortcut")
    pyautogui.hotkey('ctrl', 'c')
    time.sleep(0.5)
    capture("menu_shortcut_result", rect, check_baseline=True)
    
    observations.append({
        "test": "Menu Keyboard Shortcut",
        "action": "Pressed Ctrl+C",
        "verify": "Snackbar should appear with 'Copy action triggered' message"
    })
    
    return observations


def test_lists(rect, client_origin=None):
    """Test list selection and scrolling using element-based clicking"""
    print("\n=== LIST TESTS (Element-Based) ===")
    observations = []
    telemetry_checks = []
    
    click_pos = client_origin if client_origin else rect
    
    # Scroll sidebar back to top to ensure nav_lists is visible
    print("\n[Setup] Scrolling sidebar to top...")
    scroll_sidebar_to_top(click_pos)
    
    # Navigate to Lists section
    print("\n[Setup] Navigating to Lists section...")
    time.sleep(0.5)
    if click_element("nav_lists", click_pos):
        time.sleep(1.5)  # Longer wait for list section to spawn and render
        result = verify_telemetry_state("selected_section", "Lists")
        telemetry_checks.append(result)
        print(f"  {result['message']}")
        
        # List elements showing in telemetry
        telemetry = read_telemetry()
        list_elements = [e for e in telemetry.get("elements", []) if "list_item" in e.get("test_id", "")]
        print(f"  List elements found: {len(list_elements)}")
    else:
        print("  [SKIP] nav_lists not found")
        return observations
    
    capture("list_section", rect, check_baseline=True)
    
    # Test 1: Single selection - click first item
    print("\n[Test 1] Select list_item_0 (single selection mode)")
    if click_element("list_item_0", click_pos):
        time.sleep(0.5)
        telemetry = read_telemetry()
        selected = telemetry.get("states", {}).get("list_selected_count", "0") if telemetry else "0"
        print(f"  Selected count: {selected}")
        
        observations.append({
            "test": "List Single Selection",
            "action": "Click list_item_0",
            "passed": selected == "1",
            "verify": "One item should be selected"
        })
    
    # Test 2: Select different item (should deselect previous in single mode)
    print("\n[Test 2] Select list_item_2 (should replace previous selection)")
    if click_element("list_item_2", click_pos):
        time.sleep(0.5)
        telemetry = read_telemetry()
        selected_items = telemetry.get("states", {}).get("list_selected_items", "[]") if telemetry else "[]"
        print(f"  Selected items: {selected_items}")
        
        observations.append({
            "test": "List Selection Replace",
            "action": "Click list_item_2",
            "passed": "list_item_2" in selected_items and "list_item_0" not in selected_items,
            "verify": "Only list_item_2 should be selected (single mode)"
        })
    
    capture("list_single_selected", rect, check_baseline=True)
    
    # Test 3: Scroll the list by dragging
    print("\n[Test 3] Scroll list by dragging")
    list_bounds = get_element_bounds("list_scroll_area")
    if list_bounds:
        # Calculate scroll area center
        scroll_x = click_pos[0] + list_bounds["x"] + list_bounds["width"] / 2
        scroll_y = click_pos[1] + list_bounds["y"] + list_bounds["height"] / 2
        
        print(f"  Scrolling at ({scroll_x:.0f}, {scroll_y:.0f})")
        pyautogui.moveTo(scroll_x, scroll_y, duration=0.1)
        time.sleep(0.1)
        # Use scroll wheel to scroll down
        pyautogui.scroll(-3)  # Scroll down
        time.sleep(0.5)
        
        observations.append({
            "test": "List Scroll",
            "action": "Scroll wheel down in list area",
            "verify": "List should scroll to show more items"
        })
    
    capture("list_scrolled", rect, check_baseline=True)
    
    # Test 4: Select an item that was scrolled into view
    print("\n[Test 4] Select list_item_6 (item after scroll)")
    if click_element("list_item_6", click_pos):
        time.sleep(0.5)
        telemetry = read_telemetry()
        selected_items = telemetry.get("states", {}).get("list_selected_items", "[]") if telemetry else "[]"
        print(f"  Selected items: {selected_items}")
        
        observations.append({
            "test": "List Select After Scroll",
            "action": "Click list_item_6",
            "passed": "list_item_6" in selected_items,
            "verify": "list_item_6 should be selected"
        })
    
    passed = sum(1 for o in observations if o.get("passed", False))
    print(f"\n  Tests passed: {passed}/{len(observations)}")
    
    return observations


def is_element_visible(test_id: str, window_rect=None) -> bool:
    """Check if an element is within the visible window area"""
    bounds = get_element_bounds(test_id)
    if not bounds:
        return False
    telemetry = read_telemetry()
    if not telemetry:
        return False
    states = telemetry.get("states", {})
    win_height = float(states.get("window_height", 800))
    win_width = float(states.get("window_width", 1400))
    
    # Element is visible if its center is within the window bounds
    center_y = bounds["y"] + bounds["height"] / 2
    center_x = bounds["x"] + bounds["width"] / 2
    return 0 <= center_x <= win_width and 0 <= center_y <= win_height


def scroll_sidebar(window_rect, direction="down", amount=300):
    """Scroll the sidebar to reveal more nav items"""
    # Move to sidebar area and scroll
    win_x = window_rect[0] if window_rect else 0
    win_y = window_rect[1] if window_rect else 0
    sidebar_x = win_x + 150  # Middle of sidebar
    sidebar_y = win_y + 400  # Middle of window
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    scroll_amount = -amount if direction == "down" else amount
    pyautogui.scroll(scroll_amount // 100)  # pyautogui.scroll uses "clicks" not pixels


def scroll_sidebar_to_top(client_origin):
    """Scroll the sidebar back to the top"""
    # Move to sidebar area
    sidebar_x = client_origin[0] + 150  # Middle of sidebar
    sidebar_y = client_origin[1] + 400  # Middle of visible area
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    # Scroll up a lot to get back to top
    pyautogui.scroll(30)  # Positive = scroll up
    time.sleep(0.5)


# Track cumulative sidebar scroll offset
_sidebar_scroll_offset = 0

def reset_sidebar_scroll():
    """Reset scroll tracking"""
    global _sidebar_scroll_offset
    _sidebar_scroll_offset = 0


def scroll_sidebar_to_show_element(client_origin, element_y: float, window_height: float = 1369):
    """Scroll the sidebar to show an element at a given y position. Returns scroll offset.
    
    The sidebar has nav items starting at y=95 in layout coordinates.
    When scrolled, the visual position of an element = layout_y - scroll_offset.
    We need to scroll such that the element appears on screen (between y~100 and y~window_height-100).
    """
    global _sidebar_scroll_offset
    
    sidebar_x = client_origin[0] + 150
    sidebar_y = client_origin[1] + int(window_height * 0.4)  # 40% down the window
    
    pyautogui.moveTo(sidebar_x, sidebar_y)
    time.sleep(0.1)
    
    # Calculate where the element would appear visually with current scroll
    visual_y = element_y - _sidebar_scroll_offset
    
    # Target visible range - avoid too close to top or bottom
    visible_min = 150  # Don't click too close to top
    visible_max = window_height - 150  # Don't click too close to bottom
    
    if visual_y > visible_max:
        # Element is below visible area - need to scroll DOWN (content moves up)
        scroll_needed = visual_y - visible_max + 100  # Some padding
        scroll_clicks = max(1, int(scroll_needed / 40))  # ~40px per scroll click
        print(f"  Scrolling sidebar DOWN ({scroll_clicks} ticks) - element at visual_y={visual_y:.0f} is below {visible_max:.0f}")
        pyautogui.scroll(-scroll_clicks)  # Negative = scroll down
        _sidebar_scroll_offset += scroll_clicks * 40
        time.sleep(0.5)
    elif visual_y < visible_min:
        # Element is above visible area - need to scroll UP (content moves down)
        scroll_needed = visible_min - visual_y + 50
        scroll_clicks = max(1, int(scroll_needed / 40))
        print(f"  Scrolling sidebar UP ({scroll_clicks} ticks) - element at visual_y={visual_y:.0f} is above {visible_min:.0f}")
        pyautogui.scroll(scroll_clicks)  # Positive = scroll up
        _sidebar_scroll_offset -= scroll_clicks * 40
        _sidebar_scroll_offset = max(0, _sidebar_scroll_offset)
        time.sleep(0.5)
    
    return _sidebar_scroll_offset


def click_element_with_scroll_offset(test_id: str, client_origin, scroll_offset: float = 0) -> bool:
    """Click an element, accounting for scroll offset.
    Uses safe_click to prevent clicking outside the application."""
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"  [MISS] Element '{test_id}' not found in telemetry")
        return False
    
    center_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
    # Subtract scroll offset because scrolling moves content up on screen
    center_y = client_origin[1] + bounds["y"] + bounds["height"] / 2 - scroll_offset
    
    print(f"  Clicking '{test_id}' at ({center_x:.0f}, {center_y:.0f}) [original_y={bounds['y']:.0f}, scroll_offset={scroll_offset:.0f}]")
    
    # Use safe_click to prevent clicking outside window
    if not safe_click(center_x, center_y, f"click element '{test_id}' with scroll offset"):
        return False
    
    time.sleep(0.3)
    return True


def test_with_element_bounds(rect):
    """Test using element bounds from telemetry for precise clicking"""
    print("\n=== ELEMENT BOUNDS TESTING ===")
    observations = []
    
    # First, list available elements
    list_available_elements()
    
    # Get window dimensions from telemetry
    telemetry = read_telemetry()
    win_height = float(telemetry.get("states", {}).get("window_height", 800)) if telemetry else 800
    print(f"\nWindow height: {win_height}px")
    
    # Test 1: Navigate to VISIBLE nav items first (Checkboxes is near the top)
    print("\n[Test 1] Navigate to Checkboxes (visible nav item)")
    if is_element_visible("nav_checkboxes", rect):
        if click_element("nav_checkboxes", rect):
            time.sleep(0.5)
            result = verify_telemetry_state("selected_section", "Checkboxes")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav Click via TestId",
                "action": "Click nav_checkboxes",
                "element": "nav_checkboxes",
                "passed": result["passed"],
                "verify": "Navigation should work using element bounds"
            })
    else:
        print("  [SKIP] nav_checkboxes not visible")
    
    # Test 2: Click checkbox (should now be visible after nav)
    print("\n[Test 2] Click checkbox_0")
    time.sleep(0.5)  # Wait for telemetry update and UI to settle
    if click_element("checkbox_0", rect):
        time.sleep(0.5)
        # Check telemetry for checkbox state change
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        checkbox_toggled = any("Checkbox" in e for e in events[-5:])
        observations.append({
            "test": "Checkbox Click via TestId",
            "action": "Click checkbox_0",
            "element": "checkbox_0",
            "passed": checkbox_toggled,
            "verify": "Checkbox should toggle when clicked"
        })
    
    # Test 3: Navigate to Switches (visible)
    print("\n[Test 3] Navigate to Switches")
    time.sleep(0.5)  # Extra delay before next nav
    if is_element_visible("nav_switches", rect):
        if click_element("nav_switches", rect):
            time.sleep(0.7)  # Longer wait for navigation
            result = verify_telemetry_state("selected_section", "Switches")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav to Switches",
                "action": "Click nav_switches",
                "element": "nav_switches",
                "passed": result["passed"],
                "verify": "Should navigate to Switches section"
            })
    
    # Test 4: Click switch
    print("\n[Test 4] Click switch_0")
    time.sleep(0.5)  # Wait for switch elements to appear
    if click_element("switch_0", rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        switch_toggled = any("Switch" in e for e in events[-5:])
        observations.append({
            "test": "Switch Click via TestId",
            "action": "Click switch_0",
            "element": "switch_0",
            "passed": switch_toggled,
            "verify": "Switch should toggle when clicked"
        })
    
    # Test 5: Navigate to Radio Buttons (visible)
    print("\n[Test 5] Navigate to Radio Buttons")
    time.sleep(1.0)  # Longer delay before next nav - ensure UI has fully settled
    # Move mouse to sidebar first to ensure focus
    pyautogui.moveTo(100, 300)
    time.sleep(0.2)
    if is_element_visible("nav_radio_buttons", rect):
        if click_element("nav_radio_buttons", rect):
            time.sleep(0.8)  # Longer wait for navigation to complete
            result = verify_telemetry_state("selected_section", "RadioButtons")
            print(f"  {result['message']}")
            observations.append({
                "test": "Nav to RadioButtons",
                "action": "Click nav_radio_buttons",
                "element": "nav_radio_buttons",
                "passed": result["passed"],
                "verify": "Should navigate to RadioButtons section"
            })
    
    # Test 6: Click radio button
    print("\n[Test 6] Click radio_1")
    time.sleep(0.5)  # Wait for radio elements to appear
    if click_element("radio_1", rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        events = telemetry.get("events", []) if telemetry else []
        radio_selected = any("Radio" in e for e in events[-5:])
        observations.append({
            "test": "Radio Click via TestId",
            "action": "Click radio_1",
            "element": "radio_1",
            "passed": radio_selected,
            "verify": "Radio button should be selected"
        })
    
    # Test 7: Navigate to Sliders using element bounds
    print("\n[Test 7] Navigate to Sliders")
    time.sleep(0.7)  # Longer delay before nav
    if click_element("nav_sliders", rect):
        time.sleep(0.8)  # Longer wait for navigation
        result = verify_telemetry_state("selected_section", "Sliders")
        print(f"  {result['message']}")
        observations.append({
            "test": "Nav to Sliders",
            "action": "Click nav_sliders",
            "element": "nav_sliders",
            "passed": result["passed"],
            "verify": "Should navigate to Sliders section"
        })
    
    # Test 8: Drag slider thumb
    print("\n[Test 8] Drag slider_thumb_0")
    time.sleep(0.5)
    if drag_element("slider_thumb_0", 100, 0, rect):
        time.sleep(0.5)
        telemetry = read_telemetry()
        slider_val = telemetry.get("states", {}).get("slider_0_value", "0") if telemetry else "0"
        slider_changed = float(slider_val) > 10  # Check if slider moved from initial
        observations.append({
            "test": "Slider Drag via TestId",
            "action": "Drag slider_thumb_0 by 100px",
            "element": "slider_thumb_0",
            "passed": slider_changed,
            "verify": f"Slider value should change (got {slider_val})"
        })
    
    return observations


def run_all_tests():
    """Run all component tests"""
    print("=" * 60)
    print("BEVY MATERIAL UI - COMPONENT TESTING")
    print("=" * 60)
    
    # Delete old telemetry file
    if TELEMETRY_FILE.exists():
        TELEMETRY_FILE.unlink()
    
    # Start the app with telemetry enabled
    import os
    env = os.environ.copy()
    env['BEVY_TELEMETRY'] = '1'
    
    print("\nStarting Bevy showcase with telemetry enabled...")
    print("  (This may take a while if compilation is needed)")
    proc = subprocess.Popen(
        ["cargo", "run", "--example", "showcase", "--release"],
        cwd=WORKSPACE_DIR,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env
    )
    
    # Wait for telemetry file to appear (indicates app has started)
    print("  Waiting for application to start...")
    max_wait = 120  # 2 minutes max for compilation + startup
    waited = 0
    while waited < max_wait:
        if TELEMETRY_FILE.exists():
            # Verify it has content
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip() and '"elements"' in content:
                        print(f"  Application started after {waited}s")
                        break
            except:
                pass
        
        # Check if process failed
        if proc.poll() is not None:
            stdout, stderr = proc.communicate()
            print(f"\n  ERROR: Application failed to start!")
            print(f"  stdout: {stdout.decode()[-500:]}")
            print(f"  stderr: {stderr.decode()[-500:]}")
            return []
        
        time.sleep(1)
        waited += 1
        if waited % 10 == 0:
            print(f"  Still waiting... ({waited}s)")
    
    if waited >= max_wait:
        print("  ERROR: Timeout waiting for application to start")
        proc.terminate()
        return []
    
    # Give the UI a moment to fully render
    time.sleep(2)
    
    # Find window - returns (client_origin, window_rect)
    result = find_bevy_window()
    if not result:
        print("Could not find Bevy window!")
        proc.terminate()
        return []
    
    client_origin, window_rect = result
    
    # Use window_rect for screenshots, client_origin for element clicking
    rect = window_rect  # For backward compatibility with screenshot functions
    
    print(f"Window rect: {rect}")
    print(f"Client origin for clicks: {client_origin}")
    
    all_observations = []
    
    try:
        # Run element bounds test first (uses test_id to find elements dynamically)
        # Pass client_origin for accurate element clicking
        all_observations.extend(test_with_element_bounds(client_origin))
        
        # Then run element-based tests (pass both rect for screenshots and client_origin for clicking)
        all_observations.extend(test_nav_highlighting(rect, client_origin))
        all_observations.extend(test_checkboxes(rect, client_origin))
        all_observations.extend(test_sliders(rect, client_origin))
        all_observations.extend(test_tabs(rect, client_origin))
        all_observations.extend(test_lists(rect, client_origin))
        all_observations.extend(test_menus(rect, client_origin))
        
        # Give time for telemetry to be written
        time.sleep(1)
        
        # Read telemetry
        telemetry = read_telemetry()
        if telemetry:
            print("\n" + "=" * 60)
            print("COMPONENT TELEMETRY")
            print("=" * 60)
            print(json.dumps(telemetry, indent=2))
        else:
            print("\nNo telemetry file found - check if BEVY_TELEMETRY env var is working")
            
    finally:
        proc.terminate()
        proc.wait(timeout=5)
    
    # Print summary for AI iteration
    print("\n" + "=" * 60)
    print("TEST OBSERVATIONS FOR AI AGENT")
    print("=" * 60)
    
    for obs in all_observations:
        print(f"\n### {obs['test']}")
        print(f"Action: {obs.get('action', 'N/A')}")
        if 'verify' in obs:
            if isinstance(obs['verify'], list):
                print("Verify:")
                for v in obs['verify']:
                    print(f"  - {v}")
            else:
                print(f"Verify: {obs['verify']}")
        if 'passed' in obs:
            print(f"Passed: {obs['passed']}")
    
    print("\n" + "=" * 60)
    print("SCREENSHOTS SAVED TO:", OUTPUT_DIR)
    print("=" * 60)
    
    # Visual regression summary
    if visual_results:
        print("\n" + generate_report(visual_results))
    
    return all_observations


def run_single_component_test(component: str, maximized: bool = True):
    """Run test for a single component"""
    print("=" * 60)
    print(f"TESTING COMPONENT: {component.upper()}")
    print(f"Window mode: {'MAXIMIZED' if maximized else 'NORMAL'}")
    print("=" * 60)
    
    # Delete old telemetry file
    if TELEMETRY_FILE.exists():
        TELEMETRY_FILE.unlink()
    
    # Start the app
    import os
    env = os.environ.copy()
    env['BEVY_TELEMETRY'] = '1'
    
    print("\nStarting Bevy showcase...")
    proc = subprocess.Popen(
        ["cargo", "run", "--example", "showcase", "--release"],
        cwd=WORKSPACE_DIR,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env
    )
    
    # Wait for app to start
    print("  Waiting for application to start...")
    max_wait = 120
    waited = 0
    while waited < max_wait:
        if TELEMETRY_FILE.exists():
            try:
                with open(TELEMETRY_FILE, 'r') as f:
                    content = f.read()
                    if content.strip() and '"elements"' in content:
                        print(f"  Application started after {waited}s")
                        break
            except:
                pass
        
        if proc.poll() is not None:
            stdout, stderr = proc.communicate()
            print(f"\n  ERROR: Application failed to start!")
            print(f"  stderr: {stderr.decode()[-500:]}")
            return []
        
        time.sleep(1)
        waited += 1
        if waited % 10 == 0:
            print(f"  Still waiting... ({waited}s)")
    
    if waited >= max_wait:
        print("  ERROR: Timeout")
        proc.terminate()
        return []
    
    time.sleep(2)
    
    # Find window
    result = find_bevy_window(maximize=maximized)
    if not result:
        print("Could not find Bevy window!")
        proc.terminate()
        return []
    
    client_origin, window_rect = result
    rect = window_rect
    
    print(f"Window rect: {rect}")
    print(f"Client origin: {client_origin}")
    
    observations = []
    
    try:
        # Map component name to test function
        test_map = {
            'nav': lambda: test_nav_highlighting(rect, client_origin),
            'checkboxes': lambda: test_checkboxes(rect, client_origin),
            'sliders': lambda: test_sliders(rect, client_origin),
            'tabs': lambda: test_tabs(rect, client_origin),
            'lists': lambda: test_lists(rect, client_origin),
            'menus': lambda: test_menus(rect, client_origin),
            'bounds': lambda: test_with_element_bounds(client_origin),
        }
        
        if component == 'all':
            # Run all tests
            observations.extend(test_with_element_bounds(client_origin))
            observations.extend(test_nav_highlighting(rect, client_origin))
            observations.extend(test_checkboxes(rect, client_origin))
            observations.extend(test_sliders(rect, client_origin))
            observations.extend(test_tabs(rect, client_origin))
            observations.extend(test_lists(rect, client_origin))
            observations.extend(test_menus(rect, client_origin))
        elif component in test_map:
            observations.extend(test_map[component]())
        else:
            print(f"Unknown component: {component}")
            print(f"Available: {', '.join(test_map.keys())}, all")
        
        time.sleep(1)
        
        # Show telemetry
        telemetry = read_telemetry()
        if telemetry:
            print("\n" + "=" * 60)
            print("TELEMETRY SUMMARY")
            print("=" * 60)
            states = telemetry.get("states", {})
            print(f"  selected_section: {states.get('selected_section')}")
            print(f"  tab_selected: {states.get('tab_selected')}")
            print(f"  list_selected_items: {states.get('list_selected_items', '[]')}")
            print(f"  elements_with_bounds: {states.get('elements_with_bounds')}")
            
            # Show recent events
            events = telemetry.get("events", [])
            if events:
                print(f"\n  Recent events:")
                for e in events[-10:]:
                    print(f"    {e}")
    finally:
        proc.terminate()
        proc.wait(timeout=5)
    
    # Print results
    print("\n" + "=" * 60)
    print("TEST RESULTS")
    print("=" * 60)
    
    passed = sum(1 for o in observations if o.get('passed', False))
    failed = sum(1 for o in observations if 'passed' in o and not o['passed'])
    
    for obs in observations:
        status = "[PASS]" if obs.get('passed', False) else "[FAIL]" if 'passed' in obs else "[INFO]"
        print(f"  {status} {obs['test']}")
    
    print(f"\nSummary: {passed} passed, {failed} failed")
    
    if visual_results:
        print("\n" + generate_report(visual_results))
    
    return observations


def navigate_to_section(section_name: str, client_origin) -> bool:
    """Navigate to a specific section and verify"""
    nav_id = f"nav_{section_name.lower().replace(' ', '_')}"
    
    # Scroll sidebar to top first
    scroll_sidebar_to_top(client_origin)
    time.sleep(0.3)
    
    print(f"\nNavigating to {section_name}...")
    if click_element(nav_id, client_origin):
        time.sleep(1.0)
        result = verify_telemetry_state("selected_section", section_name)
        print(f"  {result['message']}")
        return result['passed']
    else:
        print(f"  [MISS] {nav_id} not found")
        return False


def test_navigation_only(client_origin):
    """Test navigation to all sections using position-based clicking after scrolling."""
    print("\n=== NAVIGATION-ONLY TEST ===")
    
    # Map display name to TestId (nav_xxx format used in showcase.rs)
    sections = [
        ("Buttons", "nav_buttons"),
        ("Checkboxes", "nav_checkboxes"),
        ("Switches", "nav_switches"),
        ("RadioButtons", "nav_radio_buttons"),
        ("Chips", "nav_chips"),
        ("FAB", "nav_fab"),
        ("Badges", "nav_badges"),
        ("Progress", "nav_progress"),
        ("Cards", "nav_cards"),
        ("Dividers", "nav_dividers"),
        ("Lists", "nav_lists"),
        ("Icons", "nav_icons"),
        ("IconButtons", "nav_icon_buttons"),
        ("Sliders", "nav_sliders"),
        ("TextFields", "nav_text_fields"),
        ("Dialogs", "nav_dialogs"),
        ("Menus", "nav_menus"),
        ("Tabs", "nav_tabs"),
        ("Select", "nav_select"),
        ("Snackbar", "nav_snackbar"),
        ("Tooltips", "nav_tooltips"),
        ("AppBar", "nav_app_bar"),
        ("ThemeColors", "nav_theme_colors"),
    ]
    
    # Reset sidebar to top
    scroll_sidebar_to_top(client_origin)
    time.sleep(0.5)
    
    # Get actual window dimensions from the window rect (not telemetry which is logical size)
    if _window_bounds:
        actual_window_height = _window_bounds[3] - _window_bounds[1]
        actual_client_height = _window_bounds[3] - client_origin[1]
    else:
        actual_client_height = 800
    
    print(f"Actual client height: {actual_client_height}px")
    
    # Visible area for clicking (leave margin at bottom for window chrome)
    visible_bottom = actual_client_height - 50
    
    results = []
    cumulative_scroll = 0
    
    for section_name, nav_id in sections:
        print(f"\nNavigating to {section_name}...")
        
        # Get element bounds
        bounds = get_element_bounds(nav_id)
        if not bounds:
            print(f"  [MISS] {nav_id} not found")
            results.append({"section": section_name, "passed": False})
            continue
        
        # Layout position from telemetry
        layout_y = bounds["y"]
        
        # Calculate visual position (where it appears on screen) accounting for any scroll
        visual_y = layout_y - cumulative_scroll
        
        # Calculate click position 
        click_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
        click_y = client_origin[1] + visual_y + bounds["height"] / 2
        
        # Check if click would be in bounds using actual window bounds
        click_valid, click_reason = is_click_in_bounds(click_x, click_y)
        
        if not click_valid:
            print(f"  Click would be out of bounds: {click_reason}")
            
            # Calculate how much we need to scroll to bring element into view
            # The element is below the visible area, so we need to scroll down
            if _window_bounds:
                window_bottom = _window_bounds[3]
                scroll_needed = click_y - window_bottom + 100  # Scroll to bring element into view with margin
            else:
                scroll_needed = 200  # Default scroll amount
            
            # Use the scrollbar thumb to scroll
            # The scrollbar thumb drag amount is proportional to scroll amount
            thumb_drag_amount = scroll_needed * 0.3  # Approximate ratio (thumb moves less than content)
            
            print(f"  Using scrollbar: drag thumb by ~{thumb_drag_amount:.0f}px to scroll content ~{scroll_needed:.0f}px")
            
            # Get current scroll position before
            scroll_before = get_scroll_position("sidebar_scroll_y")
            
            if drag_scrollbar("sidebar_scroll_thumb", thumb_drag_amount, client_origin):
                time.sleep(0.3)
                # Check telemetry for new scroll position
                scroll_after = get_scroll_position("sidebar_scroll_y")
                actual_scroll = scroll_after - scroll_before
                print(f"  Scroll position changed: {scroll_before:.0f} -> {scroll_after:.0f} (delta={actual_scroll:.0f})")
                
                # Update cumulative scroll with actual amount
                if actual_scroll > 0:
                    cumulative_scroll = scroll_after
                else:
                    # Scrolling didn't work - the content probably doesn't overflow
                    print(f"  [INFO] Sidebar content may not overflow - no scroll possible")
            else:
                print(f"  [INFO] Scrollbar not available or drag failed")
            
            # Recalculate visual position and click coordinates after scroll attempt
            visual_y = layout_y - cumulative_scroll
            click_y = client_origin[1] + visual_y + bounds["height"] / 2
            print(f"  After scroll: visual_y={visual_y:.0f}, cumulative_scroll={cumulative_scroll:.0f}")
        
        # Calculate click position 
        click_x = client_origin[0] + bounds["x"] + bounds["width"] / 2
        click_y = client_origin[1] + visual_y + bounds["height"] / 2
        
        print(f"  Click at ({click_x:.0f}, {click_y:.0f}) [layout_y={layout_y:.0f}, scroll={cumulative_scroll:.0f}, visual_y={visual_y:.0f}]")
        
        # Use safe_click to verify bounds
        if safe_click(click_x, click_y, f"click {nav_id}"):
            time.sleep(0.8)
            result = verify_telemetry_state("selected_section", section_name)
            print(f"  {result['message']}")
            results.append({"section": section_name, "passed": result['passed']})
        else:
            print(f"  [FAIL] Click blocked - outside window bounds")
            results.append({"section": section_name, "passed": False})
        
        time.sleep(0.2)
    
    print("\n" + "=" * 60)
    print("NAVIGATION RESULTS")
    print("=" * 60)
    
    passed = sum(1 for r in results if r['passed'])
    for r in results:
        status = "[PASS]" if r['passed'] else "[FAIL]"
        print(f"  {status} {r['section']}")
    
    print(f"\nTotal: {passed}/{len(results)} sections navigated successfully")
    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bevy Material UI Component Testing")
    parser.add_argument('component', nargs='?', default='all',
                        help='Component to test: nav, checkboxes, sliders, tabs, lists, menus, bounds, all')
    parser.add_argument('--normal', action='store_true',
                        help='Use normal window size instead of maximized')
    parser.add_argument('--nav-only', action='store_true',
                        help='Only test navigation to all sections')
    parser.add_argument('--list-elements', action='store_true',
                        help='List all available TestId elements')
    
    args = parser.parse_args()
    
    if args.nav_only:
        # Special mode: just test navigation
        print("=" * 60)
        print("NAVIGATION TEST MODE")
        print("=" * 60)
        
        if TELEMETRY_FILE.exists():
            TELEMETRY_FILE.unlink()
        
        import os
        env = os.environ.copy()
        env['BEVY_TELEMETRY'] = '1'
        
        print("\nStarting Bevy showcase...")
        proc = subprocess.Popen(
            ["cargo", "run", "--example", "showcase", "--release"],
            cwd=WORKSPACE_DIR,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env
        )
        
        # Wait for start
        waited = 0
        while waited < 120:
            if TELEMETRY_FILE.exists():
                try:
                    with open(TELEMETRY_FILE, 'r') as f:
                        if '"elements"' in f.read():
                            break
                except:
                    pass
            time.sleep(1)
            waited += 1
        
        time.sleep(2)
        result = find_bevy_window(maximize=not args.normal)
        if result:
            client_origin, _ = result
            test_navigation_only(client_origin)
        proc.terminate()
        
    elif args.list_elements:
        # List available elements from existing telemetry
        if TELEMETRY_FILE.exists():
            list_available_elements()
        else:
            print("No telemetry.json file found. Run a test first.")
    else:
        run_single_component_test(args.component, maximized=not args.normal)
