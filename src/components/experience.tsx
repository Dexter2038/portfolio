
import { Trophy } from "lucide-react";

export function Experience() {
  return (
    <section id="experience" className="p-10 h-full rounded-lg border-current border-4">
      <header className="flex items-center pb-2">
        <Trophy />
        <h1 className="text-3xl px-2">Опыт</h1>
      </header>
      <ol className="space-y-6 text-gray-700">
        <li>
          <h2 className="font-bold text-lg">🚀 Фриланс — эксперименты <span className="text-sm font-normal text-gray-500">(Апрель 2024 — Декабрь 2024)</span></h2>
          <ul className="list-disc list-inside ml-4 mt-1 space-y-1">
            <li>Провел 3 клиентских проекта через полный цикл разработки</li>
            <li>Освоил принципы оценки сроков на реальных задачах</li>
            <li>Разработал систему коммуникации с заказчиками</li>
          </ul>
        </li>
        <li>
          <h2 className="font-bold text-lg">🔒 Конфиденциальный проект <span className="text-sm font-normal text-gray-500">(Ноябрь 2024 — настоящее время)</span></h2>
          <ul className="list-disc list-inside ml-4 mt-1 space-y-1">
            <li>Разработка под NDA с ограниченным доступом к данным</li>
            <li>Работа с жёсткими дедлайнами и высокими требованиями к безопасности</li>
            <li>Еженедельные отчёты о прогрессе</li>
            <li>
              <span className="font-semibold">Результат:</span> Стабильные коммиты в течение 7+ месяцев
            </li>
          </ul>
        </li>
      </ol>
    </section>
  );
}

