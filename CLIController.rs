use WorldModel::Model;
use CLIView::View;

pub fn run() {
	let mut model = Model::new("defaultWorld.json");
	let view = View::new();
	model.add_view(view);
	println!("{}", model);
}
