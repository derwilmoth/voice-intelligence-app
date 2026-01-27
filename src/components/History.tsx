"use client";

import React, { useEffect } from "react";
import { useAppStore } from "@/lib/store";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Trash2 } from "lucide-react";
import { Badge } from "@/components/ui/badge";

export function History() {
  const { history, fetchHistory, clearHistory } = useAppStore();

  useEffect(() => {
    fetchHistory();
  }, []);

  return (
    <div className="flex flex-col h-[calc(100vh-80px)] p-4 space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-bold">History</h2>
        <Button
          variant="destructive"
          size="sm"
          onClick={() => clearHistory()}
          disabled={history.length === 0}
        >
          <Trash2 className="mr-2 h-4 w-4" /> Clear All
        </Button>
      </div>

      <ScrollArea className="flex-1 rounded-md border p-4 bg-background/50">
        {history.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
            <p>No history records found.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {history
              .slice()
              .reverse()
              .map((item) => (
                <Card key={item.id} className="overflow-hidden">
                  <CardContent className="p-4 space-y-3">
                    <div className="flex justify-between items-start">
                      <div className="space-y-1">
                        <Badge variant="secondary" className="font-mono">
                          {item.instruction}
                        </Badge>
                      </div>
                      <span className="text-xs text-muted-foreground whitespace-nowrap ml-2">
                        {new Date(item.timestamp).toLocaleString()}
                      </span>
                    </div>

                    <div className="grid gap-2 text-sm">
                      <div className="bg-muted p-2 rounded-md">
                        <p className="text-xs font-semibold mb-1 text-muted-foreground">
                          Original:
                        </p>
                        <p>{item.original_content}</p>
                      </div>
                      <div className="bg-primary/5 p-2 rounded-md border border-primary/20">
                        <p className="text-xs font-semibold mb-1 text-primary">
                          Enriched:
                        </p>
                        <p>{item.enriched_content}</p>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}
          </div>
        )}
      </ScrollArea>
    </div>
  );
}
