"use client";

import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { State } from "@/components/State";
import { Settings } from "@/components/Settings";
import { History } from "@/components/History";
import {
  Activity,
  Settings as SettingsIcon,
  History as HistoryIcon,
} from "lucide-react";
import { useAppStore } from "@/lib/store";

export default function Home() {
  const [activeTab, setActiveTab] = useState("state");
  const { fetchStatus } = useAppStore();

  const handleTabChange = (value: string) => {
    setActiveTab(value);
    // Refetch status from JSON when switching to State tab
    if (value === "state") {
      fetchStatus();
    }
  };

  return (
    <main className="h-screen w-screen bg-background text-foreground overflow-hidden">
      <Tabs
        defaultValue="state"
        value={activeTab}
        onValueChange={handleTabChange}
        className="h-full flex flex-col"
      >
        <div className="border-b px-4 py-2 bg-muted/20">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="state">
              <Activity className="w-4 h-4 mr-2" /> State
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
            value="state"
            className="h-full m-0 data-[state=active]:flex flex-col"
          >
            <State />
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
