import { Mail } from "lucide-react";

export function Contacts() {
  return (
    <section id="contacts" className="p-10 h-full rounded-lg border-current border-4">
      <header className="flex items-center pb-2">
        <Mail />
        <h1 className="text-3xl px-2">Контакты</h1>
      </header>
      <ul>
        <li><a href="https://github.com/Dexter2038">GitHub</a></li>
        <li><a href="mailto:barkaloff.m@gmail.com">Email</a></li>
      </ul>
    </section>
  )
}
