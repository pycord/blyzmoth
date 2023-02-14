use yew::prelude::*;


#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("fixed", "flex", "grid", "top-0", "z-top", "gap-2", "bg-[#2c2f33]/90", "backdrop-blur-md", "py-2", "container", "text-white")}>
            <div class={classes!("w-20", "h-20")}>
                <img src="static/pycord-logo-solid-4096.png" class={classes!("desktop:mx-60", "mobile:mx-5")} />
            </div>
        </header>
    }
}


pub fn home() -> Html {
    html! {
        <div class={classes!("relative", "font-brand", "antialiased", "overflow-hidden")}>
            <Header />
            <div>
                <div class={classes!("mobile:px-20", "bg-[#2c2f33]")}>
                    <div class={classes!("text-white", "text-center", "pt-32")}>
                        <h1 class={classes!("text-6xl", "font-semibold")}>
                            {"Efficient, Reliable, Easy —"}
                            <br/>
                            {"Pick three."}
                        </h1>
                        <h3 class={classes!("pt-10", "text-xl", "font-medium", "max-w-lg", "m-auto")}>
                            {"The Python library empowering you to make reliable and efficient Discord bots."}
                        </h3>

                    </div>
                    <div class={classes!("flex", "justify-center", "items-center", "pt-10", "text-xl", "gap-x-8", "pb-28")}>
                        <a href="https://guide.pycord.dev/introduction">
                            <button class={classes!("transition", "duration-400", "text-white", "bg-[#5865F2]", "hover:bg-[#454FBF]", "hover:scale-110", "hover:-translate-y-1", "ease-in-out", "delay-150", "rounded-full", "px-6", "py-2",)}>
                                {"Get Started"}
                            </button>
                        </a>
                        <a href="https://github.com/Pycord-Development/pycord-v3">
                            <button class={classes!("transition", "duration-400", "text-[#2c2f33]", "bg-white", "border-not-quite-black", "border", "hover:text-white", "hover:bg-[#5865F2]", "hover:border-[#5865F2]", "delay-150", "rounded-full", "px-6", "py-2",)}>
                                {"View the Source"}
                            </button>
                        </a>
                    </div>
                </div>
                <div class={classes!("mobile:px-20", "mobile:bg-[#23272a]")}>
                    <div class={classes!("bg-[#23272a]", "pt-28",)}>
                        <div class={classes!("text-white")}>
                            <h1 class={classes!("text-center", "text-4xl", "font-semibold")}>
                                {"All the features you could ever want."}
                            </h1>
                            <h3 class={classes!("text-center", "text-lg", "max-w-lg", "m-auto", "pt-5", "font-medium")}>
                                {"Want to make a small bot which sends cat images, or a powerful moderation bot? We've got you covered with everything you need."}
                            </h3>
                        </div>
                        <ul class={classes!("flex", "justify-center", "items-center", "gap-0", "pt-20", "pb-20", "flex-wrap", "text-white", "max-w-5xl", "m-auto",)}>
                            <il class={classes!("border", "border-r", "border-l", "border-b", "max-w-xs", "py-4", "px-3")}>
                                <h4 class={classes!("text-center", "font-semibold", "tracking-tight", "pb-2", "text-[#5865F2]")}>
                                    {"True Discord API Support"}
                                </h4>
                                <h5 class={classes!("max-w-sm", "text-center")}>
                                    {"The library has full support for the Discord API, with features averagely getting support early!"}
                                </h5>
                            </il>
                            <il class={classes!("border", "border-r", "border-l", "border-b", "max-w-xs", "py-4", "px-3")}>
                                <h4 class={classes!("text-center", "font-semibold", "tracking-tight", "pb-2", "text-[#5865F2]")}>
                                    {"Efficient"}
                                </h4>
                                <h5 class={classes!("max-w-sm", "text-center")}>
                                    {"The internals of our library is blazingly fast, with some parts even being written in Rust!"}
                                </h5>
                            </il>
                            <il class={classes!("border", "border-r", "border-l", "border-b", "max-w-xs", "py-4", "px-3")}>
                                <h4 class={classes!("text-center", "font-semibold", "tracking-tight", "pb-2", "text-[#5865F2]")}>
                                    {"Modern"}
                                </h4>
                                <h5 class={classes!("max-w-sm", "text-center")}>
                                    {"Pycord is"}
                                    <b class={classes!("pl-1", "pr-1")}>{"the"}</b>
                                    {"modern Discord library."}
                                    <br />
                                    {"You can build anything you can imagine in the library!"}
                                </h5>
                            </il>
                            <br />
                            <il class={classes!("border", "border-l", "border-r", "border-b", "max-w-xs", "py-4", "px-3")}>
                                <h4 class={classes!("text-center", "font-semibold", "tracking-tight", "pb-2", "text-[#5865F2]")}>
                                    {"Bot-focused"}
                                </h4>
                                <h5 class={classes!("max-w-sm", "text-center")}>
                                    {"Unlike many other legacy libraries, Pycord was made from the ground-up to be for Bots."}
                                </h5>
                            </il>
                            <il class={classes!("border", "border-l", "border-r", "border-b", "max-w-xs", "py-4", "px-3")}>
                                <h4 class={classes!("text-center", "font-semibold", "tracking-tight", "pb-2", "text-[#5865F2]")}>
                                    {"Reliable"}
                                </h4>
                                <h5 class={classes!("max-w-sm", "text-center")}>
                                    {"Pycord is incredibly reliable. Used by many of the largest Discord bots, stability is ensured."}
                                </h5>
                            </il>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
