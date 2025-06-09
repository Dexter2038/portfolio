import { Wrench } from "lucide-react";

interface Stack {
  icon: string,
  name: string
}


const stack: Stack[] = [
  {
    icon: "https://cdn.simpleicons.org/rust/000000",
    name: "Rust"
  },
  {
    icon: "https://cdn.simpleicons.org/python/3776AB",
    name: "Python"
  },
  {
    icon: "https://cdn.simpleicons.org/javascript/F7DF1E",
    name: "JavaScript"
  },
  {
    icon: "https://cdn.simpleicons.org/flutter/02569B",
    name: "Flutter"
  },
  {
    icon: "https://cdn.simpleicons.org/nextdotjs/000000",
    name: "Next.js"
  },
  {
    icon: "https://cdn.simpleicons.org/express/000000",
    name: "Express"
  },
  {
    icon: "https://cdn.simpleicons.org/react/61DAFB",
    name: "React"
  },
  {
    icon: "https://cdn.simpleicons.org/postgresql/336791",
    name: "PostgreSQL"
  },
  {
    icon: "https://cdn.simpleicons.org/docker/148CEA",
    name: "Docker"
  }
]


export function Stack() {
  return (
    <section id="stack" className="p-10 h-full rounded-lg border-current border-4">
      <span className="flex items-center pb-2">
        <Wrench />
        <h1 className="text-3xl px-2">Мой стек</h1>
      </span>
      <ol className="flex flex-col gap-1">
        {stack.map(stack => {
          return (
            <li className="flex gap-3">
              <img src={stack.icon} className="w-6 h-6" />
              <p>{stack.name}</p>
            </li>
          )
        })}
      </ol>
    </section>
  )

}
