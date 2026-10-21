import { useTranslation } from "react-i18next";

// Placeholder shell; plan 01-11 replaces it with the real layout (title bar, Durum / Ayarlar).
export default function App() {
  const { t } = useTranslation();
  return (
    <main className="flex h-full items-center justify-center">
      <h1 className="text-2xl font-semibold tracking-tight">{t("app.name")}</h1>
    </main>
  );
}
