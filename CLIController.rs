use std::rc::Rc;
use WorldModel::Model;
use CLIView::View;

pub fn run() {
	let mut model = Model::new("defaultWorld.json");
	println!("Model created");
	let view = Rc::new(View::new());
	model.add_view(view.clone());

	view.show();
}
