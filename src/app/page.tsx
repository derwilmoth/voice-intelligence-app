"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Dashboard } from "@/components/Dashboard";
import { Settings } from "@/components/Settings";
import { History } from "@/components/History";
import {
  LayoutDashboard,
  Settings as SettingsIcon,
  History as HistoryIcon,
} from "lucide-react";

export default function Home() {
  return (
    <main className="h-screen w-screen bg-background text-foreground overflow-hidden">
      <Tabs defaultValue="dashboard" className="h-full flex flex-col">
        <div className="border-b px-4 py-2 bg-muted/20">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="dashboard">
              <LayoutDashboard className="w-4 h-4 mr-2" /> Dashboard
            </TabsTrigger>
            <TabsTrigger value="history">
              <HistoryIcon className="w-4 h-4 mr-2" /> History
            </TabsTrigger>
            <TabsTrigger value="settings">
              <SettingsIcon className="w-4 h-4 mr-2" /> Settings
            </TabsTrigger>
          </TabsList>
        </div>

        <div className="flex-1 overflow-hidden">
          <TabsContent
            value="dashboard"
            className="h-full m-0 data-[state=active]:flex flex-col"
          >
            <Dashboard />
          </TabsContent>

          <TabsContent
            value="history"
            className="h-full m-0 data-[state=active]:flex flex-col"
          >
            <History />
          </TabsContent>

          <TabsContent value="settings" className="h-full m-0 overflow-y-auto">
            <Settings />
          </TabsContent>
        </div>
      </Tabs>
    </main>
  );
}
