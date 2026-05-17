import { createBrowserRouter, Navigate, Outlet } from "react-router-dom";
import { AppLayout } from "@/app/layouts/AppLayout";
import { DashboardScreen } from "@/features/dashboard/screens/DashboardScreen";
import { WorkspacesScreen } from "@/features/workspaces/screens/WorkspacesScreen";
import { ModelsScreen } from "@/features/models/screens/ModelsScreen";
import { LogsScreen } from "@/features/logs/screens/LogsScreen";
import { AccountsScreen } from "@/features/accounts/screens/AccountsScreen";
import { SettingsScreen } from "@/features/settings/screens/SettingsScreen";
import { LoginScreen } from "@/features/auth/screens/LoginScreen";
import { useAuth } from "@/features/auth/logic/AuthProvider";

function ProtectedLayout() {
  const { status } = useAuth();

  if (status === "loading") {
    return (
      <div className="min-h-screen flex items-center justify-center bg-mcm-pattern">
        <p className="text-sm text-espresso-400">验证身份...</p>
      </div>
    );
  }

  if (status === "unauthenticated") {
    return <Navigate to="/login" replace />;
  }

  return <AppLayout />;
}

function PublicOnly({ children }: { children: React.ReactNode }) {
  const { status } = useAuth();

  if (status === "loading") {
    return (
      <div className="min-h-screen flex items-center justify-center bg-mcm-pattern">
        <p className="text-sm text-espresso-400">验证身份...</p>
      </div>
    );
  }

  if (status === "authenticated") {
    return <Navigate to="/" replace />;
  }

  return <>{children}</>;
}

export const router = createBrowserRouter([{
  path: "/",
  element: <ProtectedLayout />,
  children: [
    { index: true, element: <DashboardScreen /> },
    { path: "workspaces", element: <WorkspacesScreen /> },
    { path: "models", element: <ModelsScreen /> },
    { path: "logs", element: <LogsScreen /> },
    { path: "accounts", element: <AccountsScreen /> },
    { path: "settings", element: <SettingsScreen /> },
  ],
}, {
  path: "/login",
  element: <PublicOnly><LoginScreen /></PublicOnly>,
}]);
