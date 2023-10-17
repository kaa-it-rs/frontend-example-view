use super::super::Todo;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todo_id: i32,
}

pub struct Detail {
    todo: Option<Todo>,
}

pub enum Msg {
    MakeReq(i32),
    Resp(anyhow::Result<Todo>),
}

impl Component for Detail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq(ctx.props().todo_id));
        Self { todo: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq(id) => {
                self.todo = None;
                ctx.link().send_future(async move {
                    let res = gloo_net::http::Request::get(&format!(
                        "https://jsonplaceholder.typicode.com/todos/{}",
                        id
                    ))
                    .send()
                    .await
                    .expect("can make req to jsonplaceholder");

                    let data = match res.json::<Todo>().await {
                        Ok(v) => Ok(v),
                        Err(e) => Err(anyhow::anyhow!(e)),
                    };
                    Msg::Resp(data.into())
                })
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todo = Some(data);
                }
            }
        }

        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("todo")}>
                { self.render_detail(&self.todo) }
            </div>
        }
    }
}

impl Detail {
    fn render_detail(&self, todo: &Option<Todo>) -> Html {
        match todo {
            Some(t) => {
                let completed = if t.completed {
                    Some("completed")
                } else {
                    Some("not-completed")
                };
                html! {
                    <div class={classes!("detail")}>
                        <h1>{&t.title}{" ("}<span class={classes!("id")}>{t.id}</span>{")"}</h1>
                        <div>{"by user "}{t.user_id}</div>
                        <div class={classes!(completed)}>{if t.completed { "done" } else { "not done" }}</div>
                    </div>
                }
            }
            None => {
                html! {
                    <div class={classes!("loading")}>{"loading..."}</div>
                }
            }
        }
    }
}
