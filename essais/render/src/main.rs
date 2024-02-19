fn sam() {
    println!("Hello, world!");
}

//use HtmlRenderer;
use raui_html_renderer::HtmlRenderer;
use raui_core::renderer::Renderer;

fn main(){
let sample = "dreimalschwarzer kater";
//   let html = HtmlRenderer(&sample);
   let html = Renderer::render(&sample);

   println!("{:?},html");
}