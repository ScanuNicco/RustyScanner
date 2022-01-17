fn main() {
        MainWindow::new().run();
	let artists = ["Cake", "Coldplay", "R. E. M.", "Spoon", "Red Hot Chili Peppers", "Undercover S. K. A.", "Vampire Weekend"];
}

//Maybe use the gallery demo from here as a base: https://sixtyfps.io/releases/0.1.1/editor/?load_url=https://sixtyfps.io/blog/introducing-sixtyfps/hello_world.60
sixtyfps::sixtyfps ! {
	import { Button, StandardListView, HorizontalBox, VerticalBox, TabWidget } from "sixtyfps_widgets.60";

        MainWindow := Window {
                width: 480px;
                height: 262px;
		TabWidget {
            		Tab {
                		title: "Library";
                		StandardListView {
                        		model: [{ text: "Artist names"}];
                		}
			}
			Tab {
				title: "Now Playing";
				VerticalBox{
					Text {
						text: "Song Name - Artist Name";
					}
					Text {
						text: "Album Name";
						color: #333;
					}
					HorizontalBox{
						Button {
							text: "Back";
						}
						Button {
							text: "Pause";
						}
						Button {
							text: "Forward";
						}
					}
				}
			}
		}
        }
}