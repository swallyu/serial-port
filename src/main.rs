
use native_windows_derive as nwd;
use native_windows_derive::NwgPartial;
use native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::TextBoxFlags;

#[derive(Default,NwgUi)]
pub struct SerialApp{
    #[nwg_control(size: (400, 390), position: (300, 300), title: "Basic example")]
    #[nwg_events(OnInit:[SerialApp::init] ,OnResize: [SerialApp::resize], OnWindowClose: [SerialApp::close] )]
    window:nwg::Window,

    #[nwg_layout(parent: window)]
    layout:nwg::DynLayout,

    #[nwg_control(text: "Heisenberg", size: (410, 150), position: (0, 10),flags:"VSCROLL|VISIBLE")]
    name_edit: nwg::TextBox,

    // #[nwg_control(text: "Say my name", size: (280, 60), position: (10, 40))]
    // // #[nwg_events( OnButtonClick: [SerialApp::say_hello] )]
    // hello_button: nwg::Button

    #[nwg_control(position: (0, 200), size: (402, 200),flags:"NONE|VISIBLE")]
    frame:nwg::Frame,

    #[nwg_partial(parent: frame)]
    setting:SerialSetting
}

impl SerialApp {
    fn init(&self) {
        self.layout.add_child((0, 0), (100, 100), &self.name_edit);

        self.layout.add_child((0, 100), (100, 0), &self.frame);

        self.setting.init(&self.frame);
        self.layout.fit();
    }
    fn resize(&self){
        self.layout.fit();
    }
    fn close(&self){
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default,NwgPartial)]
pub struct SerialSetting{
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(text: "John Doe", position: (10, 10), size: (100, 20))]
    // #[nwg_events(OnChar: [print_char(EVT_DATA)])]
    name_input: nwg::TextInput,

    #[nwg_control(text: "75", flags: "NUMBER|VISIBLE", position: (10, 40), size: (100, 20))]
    age_input: nwg::TextInput,

    #[nwg_control(text: "Programmer", position: (10, 70), size: (100, 25))]
    job_input: nwg::TextInput,
}

impl SerialSetting{
    fn init(&self,frame: &nwg::Frame){
        self.layout.parent(frame);

        self.layout.add_child((0, 0), (0, 0), &self.name_input);
        self.layout.add_child((0, 0), (0, 0), &self.age_input);
        self.layout.add_child((0, 0), (0, 0), &self.job_input);

    }
}

fn main() {
    nwg::init().expect("fail init native window gui.");

    let _app = SerialApp::build_ui(Default::default()).expect("fail build app ui.");

    nwg::dispatch_thread_events();
}
