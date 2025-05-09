use windows::core::Result;
use windows::Win32::System::Console;
use std::io::{stdout, Write};

pub fn get_cursor() -> Result<(i16, i16)> {
    let mut info: Console::CONSOLE_SCREEN_BUFFER_INFO = Console::CONSOLE_SCREEN_BUFFER_INFO::default();
    unsafe{
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE)?;
        Console::GetConsoleScreenBufferInfo (handle, &mut info)?;
    }

    Ok((info.dwCursorPosition.X, info.dwCursorPosition.Y))
}

pub fn set_cursor(x: i16, y: i16) {
    unsafe{
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE).expect("Failed to get handle");
        Console::SetConsoleCursorPosition(handle, Console::COORD{
            X: x,
            Y: y,
        }).expect("Failed to set cursor position");
    }
}

pub fn update_screen(x: i16, y: i16, c: String) {
    set_cursor(x, y);
    let mut lock = stdout().lock();
    write!(lock, "{}", c).unwrap();
    stdout().flush().expect("Failed to flush stdout");
}

pub fn get_user_input() -> String{
    let mut read_buffer: [Console::INPUT_RECORD; 32] = [Console::INPUT_RECORD::default(); 32];
    let mut write_buffer: [Console::INPUT_RECORD; 1] = [Console::INPUT_RECORD::default(); 1];
    write_buffer[0].EventType = Console::KEY_EVENT as u16;
    write_buffer[0].Event.KeyEvent = Console::KEY_EVENT_RECORD {
        bKeyDown: windows::core::BOOL(1), // Key is pressed
        wRepeatCount: 1,
        wVirtualKeyCode: 0,
        wVirtualScanCode: 0,
        uChar: Console::KEY_EVENT_RECORD_0 { UnicodeChar: '+' as u16 }, // Character 'j'
        dwControlKeyState: 0,
    };
    let mut read_count: u32 = 0;
    let mut write_count: u32 = 0;
    unsafe {
        let handle = Console::GetStdHandle(Console::STD_INPUT_HANDLE).expect("Failed to get handle");
        // put something in stdin (in order to not pause)
        Console::WriteConsoleInputA(handle, &mut write_buffer, &mut write_count).expect("Failed to write input");

        // receive input from stdin
        Console::ReadConsoleInputW(handle, &mut read_buffer, &mut read_count).expect("Failed to read input");
    }
    
    let mut result = String::new();
    for record in &read_buffer[..read_count as usize - 1] {
        if record.EventType == Console::KEY_EVENT as u16 {
            let key_event = unsafe { record.Event.KeyEvent };
            if key_event.bKeyDown.as_bool() {
                unsafe {
                    if let Some(ch) = char::from_u32(key_event.uChar.UnicodeChar as u32) {
                        result.push(ch);
                    }
                }
            }
        }
    }
    result
}

pub fn set_console_utf8() {
    unsafe {
        Console::SetConsoleOutputCP(65001).expect("can't set CP"); // Set output code page to UTF-8
        Console::SetConsoleCP(65001).expect("can't set CP");
    }
}

pub fn enable_virtual_terminal_processing() {
    unsafe {
        use windows::Win32::System::Console;
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE).expect("Failed to get handle");

        let mut mode = Console::CONSOLE_MODE::default();
        if Console::GetConsoleMode(handle, &mut mode).is_err() {
            return; // Not a console
        }

        // ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004
        let new_mode = mode | Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        let _ = Console::SetConsoleMode(handle, new_mode);
    }
}

pub fn clear_console_windows() {
    unsafe {
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE).unwrap();

        // Get console info to find the size of the screen buffer
        let mut console_info = Console::CONSOLE_SCREEN_BUFFER_INFO::default();
        Console::GetConsoleScreenBufferInfo(handle, &mut console_info).unwrap();

        let console_size = console_info.dwSize.X as u32 * console_info.dwSize.Y as u32;
        let coord = Console::COORD { X: 0, Y: 0 };

        // Fill the console with spaces
        let mut chars_written = 0;
        Console::FillConsoleOutputCharacterA(handle, b' ' as i8, console_size, coord, &mut chars_written).unwrap();

        // Reset the attributes
        Console::FillConsoleOutputAttribute(
            handle,
            console_info.wAttributes.0,
            console_size,
            coord,
            &mut chars_written
        ).unwrap();

        // Move cursor to top-left corner
        Console::SetConsoleCursorPosition(handle, coord).unwrap();
    }
}