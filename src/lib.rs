use serde_derive::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

mod todo;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/todo/:id")]
    Detail { id: i32 },
    #[at("/")]
    Home,
}

pub struct Home {
    todos: Option<Vec<Todo>>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

pub enum Msg {
    MakeReq,
    Resp(anyhow::Result<Vec<Todo>>),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);
        Self {
            todos: None,
            // fetch_task: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                self.todos = None;
                ctx.link().send_future(async {
                    let res =
                        gloo_net::http::Request::get("https://jsonplaceholder.typicode.com/todos")
                            .send()
                            .await
                            .expect("can make req to jsonplaceholder");

                    let data = match res.json::<Vec<Todo>>().await {
                        Ok(v) => Ok(v),
                        Err(e) => Err(anyhow::anyhow!(e)),
                    };
                    Msg::Resp(data.into())
                })
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                }
            }
        }

        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let todos = self.todos.clone();
        let cb = ctx.link().callback(|_| Msg::MakeReq);

        html! {
            <div class={classes!("todo")}>
                <div>
                    <div class={classes!("refresh")}>
                        <button onclick={cb.clone()}>
                            {"refresh"}
                        </button>
                    </div>
                    <todo::list::List todos={todos.clone()} />
                </div>
            </div>
        }
    }
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Detail { id } => html! { <todo::detail::Detail todo_id={id} /> },
    }
}

#[function_component(TodoApp)]
fn app() -> Html {
    html! {
        <BrowserRouter>
          <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

pub fn run_app() {
    yew::Renderer::<TodoApp>::new().render();
}
