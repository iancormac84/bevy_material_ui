"""
Test element clicking using telemetry bounds
"""
import json
import time
import subprocess
import sys
from pathlib import Path

try:
    import pyautogui
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "pyautogui"], check=True)
    import pyautogui

try:
    import ctypes
except ImportError:
    pass

pyautogui.FAILSAFE = False
pyautogui.PAUSE = 0.1

TELEMETRY_FILE = Path("c:/github/m2iab_games/bevy_material_ui/telemetry.json")


def read_telemetry():
    if TELEMETRY_FILE.exists():
        try:
            return json.load(open(TELEMETRY_FILE))
        except Exception as e:
            print(f"Error: {e}")
    return None


def get_element_bounds(test_id):
    data = read_telemetry()
    if not data:
        return None
    for elem in data.get("elements", []):
        if elem.get("test_id") == test_id:
            return elem
    return None


def find_bevy_window():
    """Find, maximize, and return the Bevy window's client area position"""
    try:
        user32 = ctypes.windll.user32
        
        # Constants for ShowWindow
        SW_MAXIMIZE = 3
        SW_RESTORE = 9
        
        windows = []
        def callback(hwnd, _):
            # Check if window is visible
            if not user32.IsWindowVisible(hwnd):
                return True
                
            length = user32.GetWindowTextLengthW(hwnd)
            if length > 0:
                title = ctypes.create_unicode_buffer(length + 1)
                user32.GetWindowTextW(hwnd, title, length + 1)
                title_str = title.value
                # Match the showcase window by its actual title
                if "Material Design 3" in title_str:
                    windows.append({
                        "hwnd": hwnd,
                        "title": title_str,
                    })
            return True
        
        WNDENUMPROC = ctypes.WINFUNCTYPE(ctypes.c_bool, ctypes.c_int, ctypes.c_int)
        user32.EnumWindows(WNDENUMPROC(callback), 0)
        
        if windows:
            win = windows[0]
            hwnd = win["hwnd"]
            print(f"Found window: {win['title']}")
            
            # Restore if minimized, then maximize
            user32.ShowWindow(hwnd, SW_RESTORE)
            time.sleep(0.2)
            user32.ShowWindow(hwnd, SW_MAXIMIZE)
            time.sleep(0.3)
            
            # Focus window
            user32.SetForegroundWindow(hwnd)
            time.sleep(0.3)
            
            # Get window rect after maximizing
            rect = ctypes.wintypes.RECT()
            user32.GetWindowRect(hwnd, ctypes.byref(rect))
            
            # Skip if still minimized (position < -10000)
            if rect.left < -10000 or rect.top < -10000:
                print("  Window appears minimized, skipping")
                return None
                
            # Get client area offset (title bar etc)
            client_point = ctypes.wintypes.POINT(0, 0)
            user32.ClientToScreen(hwnd, ctypes.byref(client_point))
            
            print(f"  Window maximized at: ({rect.left}, {rect.top}, {rect.right}, {rect.bottom})")
            print(f"  Client origin: ({client_point.x}, {client_point.y})")
            
            # Return client area position (where Bevy coordinates start)
            return (client_point.x, client_point.y)
        else:
            print("No Bevy window found (looking for 'Material Design 3' in title)")
    except Exception as e:
        print(f"Window search failed: {e}")
    return None


def list_elements():
    data = read_telemetry()
    if not data:
        print("No telemetry")
        return
    elements = data.get("elements", [])
    print(f"\n{len(elements)} elements with TestId:")
    for e in sorted(elements, key=lambda x: x.get("test_id", "")):
        print(f"  {e['test_id']:25s} x={e['x']:6.0f} y={e['y']:6.0f} w={e['width']:5.0f} h={e['height']:5.0f}")


def click_element(test_id, window_rect=None):
    bounds = get_element_bounds(test_id)
    if not bounds:
        print(f"Element '{test_id}' not found")
        return False
    
    win_x = window_rect[0] if window_rect else 0
    win_y = window_rect[1] if window_rect else 0
    
    # Calculate center of element
    center_x = win_x + bounds["x"] + bounds["width"] / 2
    center_y = win_y + bounds["y"] + bounds["height"] / 2
    
    print(f"Clicking '{test_id}' at screen ({center_x:.0f}, {center_y:.0f})")
    pyautogui.click(center_x, center_y)
    time.sleep(0.3)
    return True


def main():
    print("Element Click Test")
    print("=" * 40)
    
    # Find window (returns client area origin as (x, y) tuple)
    client_pos = find_bevy_window()
    if not client_pos:
        print("Bevy window not found, using (0, 0) offset")
        client_pos = (0, 0)
    
    # List elements
    list_elements()
    
    # Test clicking on a nav item
    print("\n--- Test: Click nav_checkboxes ---")
    if click_element("nav_checkboxes", client_pos):
        time.sleep(0.5)
        
        # Verify by checking telemetry
        data = read_telemetry()
        if data:
            nav_selected = data.get("states", {}).get("nav_selected")
            print(f"Result: nav_selected = {nav_selected}")
            if nav_selected == "Checkboxes":
                print("[PASS] Navigation worked!")
            else:
                print(f"[FAIL] Expected 'Checkboxes', got '{nav_selected}'")
    
    print("\nTest complete")


if __name__ == "__main__":
    import ctypes.wintypes
    main()
