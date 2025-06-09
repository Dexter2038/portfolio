import { Profile } from "@/components/profile"
import { Process } from "@/components/process"
import { Experience } from "@/components/experience"
import { Stack } from "@/components/stack"
import { Projects } from "@/components/projects"
import { Contacts } from "@/components/contacts"
import { CTA } from "@/components/cta"



export function Main() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-3 p-1 md:p-3">
      {/* Top Row */}
      <div>
        <Profile />
      </div>
      <div>
        <Stack />
      </div>
      <div>
        <Process />
      </div>

      {/* Middle Row */}
      <div>
        <Experience />
      </div>
      <div>
        <Projects />
      </div>
      <div>
        <Contacts />
      </div>

      {/* Bottom CTA Row */}
      <div className="md:col-span-3">
        <CTA />
      </div>
    </div>
  )
}


