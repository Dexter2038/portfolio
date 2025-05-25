import { Header } from "@/components/header"
import { Greeting } from "@/components/greeting"
import { Nav } from "@/components/nav"
import { Stack } from "@/components/stack"
import { Projects } from "@/components/projects"
import { Contacts } from "@/components/contacts"

export function Main() {
  return (<>
    <Header />
    <Greeting />
    <Nav />
    <Stack />
    <Projects />
    <Contacts />
  </>)
}
