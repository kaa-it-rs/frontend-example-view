use yew::prelude::*;

use super::super::Todo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todos: Option<Vec<Todo>>,
}

pub struct List {}

pub enum Msg {}

impl Component for List {
    type Properties = Props;
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.render_list(&ctx.props().todos)}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
}

impl List {
    fn render_list(&self, todos: &Option<Vec<Todo>>) -> Html {
        if let Some(t) = todos {
            html! {
                <div class={classes!("list")}>
                    { t.iter().map(|todo| self.view_todo(todo)).collect::<Html>()}
                </div>
            }
        } else {
            html! {
                <div class={classes!("loading")}>{"loading..."}</div>
            }
        }
    }

    fn view_todo(&self, todo: &Todo) -> Html {
        let completed = if todo.completed {
            Some("completed")
        } else {
            None
        };

        html! {
            <div class={classes!("list-item", completed)}>
                {&todo.title}
            </div>
        }
    }
}
