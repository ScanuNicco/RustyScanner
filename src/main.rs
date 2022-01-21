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
                title: "Tamarack Music Player";
                
                Library := VerticalLayout {
                    Rectangle {
                        color: #eee;
                        HorizontalBox {
                            Button {
                                text: "Shuffle All";
                            }
                            Text {
                                text: "Library";
                                horizontal-alignment: center;
                                vertical-alignment: center;
                                font-size: 14pt;
                                font-weight: 500;
                            }
                            Button {
                                text: "Now Playing";
                                clicked => {
                                    Library.visible = false;
                                    Player.visible = true;
                                }
                            }
                        }
                    }
                    StandardListView {
                            border-width: 0px;
                            model: [{ text: "Artist name"}, { text: "Artist name"}, { text: "Artist name"},{ text: "Artist name"}, { text: "Artist name"}, { text: "Artist name"}, { text: "Artist name"}, { text: "Artist name"}, { text: "Artist name"}];
                    }
                }
                Player := VerticalBox{
                    visible: false;
                    Button {
                                text: "Library";
                                clicked => {
                                    Library.visible = true;
                                    Player.visible = false;
                                }
                                width: 100px;
                                height: 25px;
                    }
                    Text {
                        text: "Song Name - Artist Name";
                        horizontal-alignment: center;
                        font-size: 18pt;
                        font-weight: 500;
                    }
                    Text {
                        text: "Album Name";
                        horizontal-alignment: center;
                        font-size: 16pt;
                        font-weight: 300;
                        color: #555;
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
