import { BriefcaseBusiness } from "lucide-react";

export function Projects() {
  return (
    <section id="projects" className="p-10 h-full rounded-lg border-current border-4">
      <span className="flex items-center pb-2">
        <BriefcaseBusiness />
        <h1 className="text-3xl px-2">Проекты</h1>
      </span>
      <ul className="flex gap-3">
        <li className="border-2 rounded border-current">
          <article >
            <header>
              <h3>Название</h3>
              <p>Описание</p>
            </header>

            <section>
              <h3>Технологии:</h3>
              <ul>
                <li>1 стек</li>
                <li>2 стек</li>
              </ul>
            </section>

            <footer>
              <a href="https://github.com">Github</a>
              <a href="https://demo.com">Demo</a>
            </footer>
          </article>

        </li>

        <li className="border-2 rounded border-current">
          <article>
            <header>
              <h3>Название</h3>
              <p>Описание</p>
            </header>

            <section>
              <h3>Технологии:</h3>
              <ul>
                <li>1 стек</li>
                <li>2 стек</li>
              </ul>
            </section>

            <footer>
              <a href="https://github.com">Github</a>
              <a href="https://demo.com">Demo</a>
            </footer>
          </article>
        </li>

        <li className="border-2 rounded border-current">
          <article>
            <header>
              <h3>Название</h3>
              <p>Описание</p>
            </header>

            <section>
              <h3>Технологии:</h3>
              <ul>
                <li>1 стек</li>
                <li>2 стек</li>
              </ul>
            </section>

            <footer>
              <a href="https://github.com">Github</a>
              <a href="https://demo.com">Demo</a>
            </footer>
          </article>
        </li>

      </ul>
    </section>
  )

}
