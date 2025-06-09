import { GitCompareArrows } from "lucide-react";

export function Process() {
  return (
    <section id="process" className="p-10 h-full rounded-lg border-current border-4">
      <header className="flex items-center pb-2">
        <GitCompareArrows />
        <h1 className="text-3xl px-2">Процесс</h1>
      </header>
      <ol className="list-decimal list-inside space-y-3">
        <li><strong>Планирование:</strong> Понимаю задачи и создаю план.</li>
        <li><strong>Разработка:</strong> Пишу чистый и тестируемый код.</li>
        <li><strong>Тестирование:</strong> Автоматические и ручные тесты.</li>
        <li><strong>Деплой:</strong> Настраиваю CI/CD для автоматических релизов.</li>
        <li><strong>Итерации:</strong> Улучшаю продукт на основе отзывов.</li>
      </ol>
    </section>
  )
}
