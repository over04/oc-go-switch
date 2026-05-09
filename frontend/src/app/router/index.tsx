import { createBrowserRouter } from "react-router-dom";
import { AppLayout } from "@/app/layouts/AppLayout";
import { DashboardScreen } from "@/features/dashboard/screens/DashboardScreen";
import { KeysScreen } from "@/features/keys/screens/KeysScreen";
import { ModelsScreen } from "@/features/models/screens/ModelsScreen";
import { LogsScreen } from "@/features/logs/screens/LogsScreen";
import { AccountsScreen } from "@/features/accounts/screens/AccountsScreen";
import { SettingsScreen } from "@/features/settings/screens/SettingsScreen";

export const router = createBrowserRouter([{
  path: "/",
  element: <AppLayout />,
  children: [
    { index: true, element: <DashboardScreen /> },
    { path: "keys", element: <KeysScreen /> },
    { path: "models", element: <ModelsScreen /> },
    { path: "logs", element: <LogsScreen /> },
    { path: "accounts", element: <AccountsScreen /> },
    { path: "settings", element: <SettingsScreen /> },
  ],
}]);
