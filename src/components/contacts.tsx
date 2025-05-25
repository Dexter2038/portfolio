import { Mail } from "lucide-react";

export function Contacts() {
  return (
    <section id="contacts" className="p-10 border-t-current border-t-4">
      <span className="flex items-center pb-2">
        <Mail />
        <h1 className="text-3xl px-2">Контакты</h1>
      </span>
      <ul>
        <li>Github</li>
        <li>Email</li>
      </ul>
    </section>
  )
}
