import { Wrench } from "lucide-react";

export function Stack() {
  return (
    <section id="stack" className="p-10 h-full rounded-lg border-current border-4">
      <span className="flex items-center pb-2">
        <Wrench />
        <h1 className="text-3xl px-2">Мой стек</h1>
      </span>
      <ol>
        <li>
          <p>
            Rust
          </p>
          <p>
            Python
          </p>
          <p>
            JavaScript
          </p>
        </li>
      </ol>
    </section>
  )

}
