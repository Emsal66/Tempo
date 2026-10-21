import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import tr from "./tr.json";

i18n.use(initReactI18next).init({
  lng: "tr",
  fallbackLng: "tr",
  supportedLngs: ["tr"],
  resources: { tr: { translation: tr } },
  interpolation: { escapeValue: false },
  returnNull: false,
  initAsync: false,
});

export default i18n;
