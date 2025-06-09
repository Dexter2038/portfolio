import { Mail } from "lucide-react";

interface Contact {
  link: string;
  name: string;
  value: string;
}

const contacts: Contact[] = [
  {
    link: "https://github.com/Dexter2038",
    name: "GitHub",
    value: "https://github.com/Dexter2038",
  },
  {
    link: "mailto:barkaloff.m@gmail.com",
    name: "Email",
    value: "barkaloff.m@gmail.com",
  },
  {
    link: "https://t.me/+79304178085",
    name: "Telegram",
    value: "+79304178085",
  },
  {
    link: "https://wa.me/+79304178085",
    name: "WhatsApp",
    value: "+79304178085",
  },
  {
    link: "tel:+79304178085",
    name: "Phone",
    value: "+79304178085",
  }
]

export function Contacts() {
  return (
    <section id="contacts" className="p-10 h-full rounded-lg border-current border-4">
      <header className="flex items-center pb-2">
        <Mail />
        <h1 className="text-3xl px-2">Контакты</h1>
      </header>
      <ul>
        {contacts.map(contact => (
          <li><a href={contact.link}><strong>{contact.name}</strong> - {contact.value}</a></li>
        ))}
      </ul>
    </section>
  )
}
