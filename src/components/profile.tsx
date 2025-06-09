import { LocateIcon } from "lucide-react";

export function Profile() {
  return (
    <div className="flex justify-around w-full">
      <img src="profile.jpg" className="aspect-square w-1/2 rounded-xl" />
      <div className="flex flex-col-reverse gap-3 items-center justify-center">
        <h1 className="text-center">Вдохновлённый разработчик</h1>
        <div className="flex gap-3">
          <LocateIcon />
          <p className="text-center">Россия, Москва</p>
        </div>
      </div>
    </div>
  )
}
