use yew::prelude::*;

enum Msg {
    AddItem { description: String },
    Complete { id: String },
}

struct ItemList {
    link: ComponentLink<Self>,
    items: Vec<Item>,
}

struct Item {
    description: String,
    id: String,
}

impl Component for ItemList {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem { description } => {
                self.items.push(Item {
                    description,
                    id: "0".into(),
                });
                true
            }
            Msg::Complete { id } => {
                let pos = self.items.iter().position(|it| it.id == id);
                if let Some(pos) = pos {
                    self.items.remove(pos);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let complete_callback = |item: Item| Msg::Complete { id: item.id };
        // let add_callback = |description| Msg::AddItem { description };

        let items = self
            .items
            .iter()
            .map(|it| {
                let id = it.id.clone();
                let callback = move |_| Msg::Complete { id: id.clone() };
                html! { <p>
                { &it.description }
                <button onclick=self.link.callback(callback)>
                    { "Done" }
                </button> </p> }
            })
            .collect::<Html>();
        html! {
            <div>
                <ul class="item-list">
                    { items }
                </ul>
                <input class="add-input"/>
                <button onclick=self.link.callback(|_| Msg::AddItem{description: "test".into()})>{ "Add" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<ItemList>()
}
