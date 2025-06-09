import { Profile } from "@/components/profile"
import { Process } from "@/components/process"
import { Experience } from "@/components/experience"
import { Stack } from "@/components/stack"
import { Projects } from "@/components/projects"
import { Contacts } from "@/components/contacts"
import { CTA } from "@/components/cta"

export function Main() {
  return (
    <div>
      <Profile />
      <Stack />
      <Process />
      <Experience />
      <Projects />
      <Contacts />
      <CTA />
    </div>
  )
}
