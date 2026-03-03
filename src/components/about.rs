use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-20 px-4 max-w-4xl mx-auto">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-4xl font-semibold mb-6 tracking-tight">
                    About Me
                </h2>
                <p class="text-xl text-center max-w-2xl mx-auto">
                    A passionate systems engineer with a background in medical
                    image classification using deep learning&apos;s U-net
                    architectures and a solid foundation in low level system
                    design focusing on the hardware/software interface while
                    pursuing a degree in Medical Engineering.
                </p>
            </div>

            <div class="grid md:grid-cols-2 gap-12 items-start">
                <div class="space-y-8">
                    <div>
                        <h3 class="text-xl font-semibold mb-3">
                            Background
                        </h3>
                        <p>-</p>
                        <p class="text-secondary leading-relaxed">
                            I am a fifth year student of Medical Engineering at
                            Technical University of Mombasa. My skillset
                            encompasses deep learning architectures and
                            compilers, compiler design targeting modern
                            architectures with given parrallelism, circuit
                            analysis and embedded systems.
                        </p>
                        -
                        <br />
                        <p class="text-secondary leading-relaxed">
                            I am currently at{" "}
                            <a
                                className="text-blue-600 hover:text-yellow-
                                00 hover:no-underline transition-colors"
                                href="https://www.zone01kisumu.ke"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                Zone01 Kisumu
                            </a>
                            <text> </text>
                            where I&apos;m a Software Developer Apprentice with
                            a focus on Machine Learning Systems.
                        </p>
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-6">
                    <div class="text-center p-4">
                        <div class="text-amber-400 text-4xl font-bold mb-2">
                            2+
                        </div>
                        <div class="text-amber-400 text-sm">
                            Years Experience
                        </div>
                    </div>

                    {/*<div class="text-center p-4">
                        <div class="text-3xl font-bold text-primary mb-2">
                            10+
                        </div>
                        <div class="text-secondary text-sm">
                            Projects Built
                        </div>
                    </div>*/}

                    <div class="">
                        {/*<div className="text-amber-400 text-3xl font-bold mb-2">
                            24/7
                        </div>*/}
                        {/*<div className="text-amber-400 text-sm">
                            Learning Mode
                        </div>*/}
                    </div>
                </div>
            </div>
        </section>
    }
}
