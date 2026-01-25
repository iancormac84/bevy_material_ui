# DateTime Picker Refactor Plan

## Executive Summary

The current `datetime_picker.rs` implementation combines date and time selection in a single component, which doesn't match Material Design 3 specifications. This document outlines the complete refactoring plan to align with the reference Material Components behavior.

## Critical Issues Identified

### 1. **Architectural Mismatch**
- ❌ Current: Combined date+time picker in one dialog
- ✅ Required: Separate MaterialDatePicker and MaterialTimePicker components
- **Impact**: Violates Material Design 3 specifications, confusing UX

### 2. **Missing Input Modes**
-  Date Picker needs: CALENDAR ↔ TEXT INPUT modes
-  Time Picker needs: CLOCK ↔ KEYBOARD modes
- **Impact**: Poor accessibility, no power-user workflows

### 3. **Time Picker Uses Buttons Instead of Clock Face**
- ❌ Current: +/- increment buttons
- ✅ Required: Radial clock face with draggable hand
- **Impact**: Non-standard UI, poor touch interaction

### 4. **No Date Range Selection**
- ❌ Current: Single date only
- ✅ Required: Single date OR date range selection
- **Impact**: Cannot build apps requiring date range (booking, analytics, etc.)

## Refactoring Strategy

### Phase 1: Foundation (Week 1)
**Goal**: Create separate, properly architected components

#### 1.1 Create New File Structure
```
src/
  date_picker/
    mod.rs              # Public API
    calendar.rs         # Calendar presenter
    text_input.rs       # Text input presenter  
    constraints.rs      # Date validation
    range_selector.rs   # Date range logic
  time_picker/
    mod.rs              # Public API
    clock.rs            # Clock face presenter
    keyboard.rs         # Keyboard input presenter
    format.rs           # 12/24H handling
  datetime_picker.rs    # DEPRECATED (compatibility shim)
```

#### 1.2 Define Core Types
```rust
// Date Picker
pub enum DatePickerMode { Single, Range }
pub enum DateInputMode { Calendar, Text }
pub struct MaterialDatePicker { /* ... */ }
pub trait DateSelector: Send + Sync {
    This document has been removed. See docs/README.md for user-facing documentation.
    fn set_selection(&mut self, selection: DateSelection);
