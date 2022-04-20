use pancurses;
use pancurses::Input;

pub fn report_chars(window: &pancurses::Window) -> (Input, Input) {
    let mut inputs = vec![
        Input::KeyCodeYes,
        Input::KeyBreak,
        Input::KeyDown,
        Input::KeyUp,
        Input::KeyLeft,
        Input::KeyRight,
        Input::KeyHome,
        Input::KeyBackspace,
        Input::KeyF0,
        Input::KeyF1,
        Input::KeyF2,
        Input::KeyF3,
        Input::KeyF4,
        Input::KeyF5,
        Input::KeyF6,
        Input::KeyF7,
        Input::KeyF8,
        Input::KeyF9,
        Input::KeyF10,
        Input::KeyF11,
        Input::KeyF12,
        Input::KeyF13,
        Input::KeyF14,
        Input::KeyF15,
        Input::KeyDL,
        Input::KeyIL,
        Input::KeyDC,
        Input::KeyIC,
        Input::KeyEIC,
        Input::KeyClear,
        Input::KeyEOS,
        Input::KeyEOL,
        Input::KeySF,
        Input::KeySR,
        Input::KeyNPage,
        Input::KeyPPage,
        Input::KeySTab,
        Input::KeyCTab,
        Input::KeyCATab,
        Input::KeyEnter,
        Input::KeySReset,
        Input::KeyReset,
        Input::KeyPrint,
        Input::KeyLL,
        Input::KeyAbort,
        Input::KeySHelp,
        Input::KeyLHelp,
        Input::KeyBTab,
        Input::KeyBeg,
        Input::KeyCancel,
        Input::KeyClose,
        Input::KeyCommand,
        Input::KeyCopy,
        Input::KeyCreate,
        Input::KeyEnd,
        Input::KeyExit,
        Input::KeyFind,
        Input::KeyHelp,
        Input::KeyMark,
        Input::KeyMessage,
        Input::KeyMove,
        Input::KeyNext,
        Input::KeyOpen,
        Input::KeyOptions,
        Input::KeyPrevious,
        Input::KeyRedo,
        Input::KeyReference,
        Input::KeyRefresh,
        Input::KeyReplace,
        Input::KeyRestart,
        Input::KeyResume,
        Input::KeySave,
        Input::KeySBeg,
        Input::KeySCancel,
        Input::KeySCommand,
        Input::KeySCopy,
        Input::KeySCreate,
        Input::KeySDC,
        Input::KeySDL,
        Input::KeySelect,
        Input::KeySEnd,
        Input::KeySEOL,
        Input::KeySExit,
        Input::KeySFind,
        Input::KeySHome,
        Input::KeySIC,
        Input::KeySLeft,
        Input::KeySMessage,
        Input::KeySMove,
        Input::KeySNext,
        Input::KeySOptions,
        Input::KeySPrevious,
        Input::KeySPrint,
        Input::KeySRedo,
        Input::KeySReplace,
        Input::KeySRight,
        Input::KeySResume,
        Input::KeySSave,
        Input::KeySSuspend,
        Input::KeySUndo,
        Input::KeySuspend,
        Input::KeyUndo,
        Input::KeyResize,
        Input::KeyEvent,
        Input::KeyMouse,
        Input::KeyA1,
        Input::KeyA3,
        Input::KeyB2,
        Input::KeyC1,
        Input::KeyC3,
    ];
    inputs.append(
        &mut (0..=255)
            .map(|i| Input::Character(char::from_u32(i).unwrap()))
            .collect::<Vec<Input>>(),
    );

    loop {
        let input = window.getch();

        for i in inputs.iter() {
            if let Some(input) = input {
                if input == *i {
                    window.addstr(&format!("{:?}", input));
                }
            }
        }
    }
}
