import { Trophy } from "lucide-react";

export function Experience() {
  return (
    <section id="experience" className="p-10 h-full rounded-lg border-current border-4">
      <header className="flex items-center pb-2">
        <Trophy />
        <h1 className="text-3xl px-2">Опыт</h1>
      </header>
      <ol className="list-decimal list-inside space-y-3 text-gray-700">
        <li><strong>6 месяцев фриланса:</strong> Работал с разными клиентами, быстро решал задачи, улучшал коммуникацию и тайм-менеджмент.</li>
        <li><strong>6 месяцев проекта, о котором нельзя говорить:</strong> Закрытая разработка, работа с конфиденциальными данными, строгие дедлайны, высокая ответственность.</li>
      </ol>
    </section>
  )
}
