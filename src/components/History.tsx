"use client";

import React, { useEffect, useState } from "react";
import { useAppStore } from "@/lib/store";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Trash2, RefreshCw, X, Copy } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

export function History() {
  const { history, fetchHistory, clearHistory, deleteHistoryItem } =
    useAppStore();
  const [showClearDialog, setShowClearDialog] = useState(false);

  useEffect(() => {
    fetchHistory();
  }, []);

  const handleClearAll = () => {
    clearHistory();
    setShowClearDialog(false);
  };

  return (
    <div className="flex flex-col h-screen">
      <div className="flex justify-between items-center px-4 pt-4 pb-3 shrink-0">
        <h2 className="text-xl font-bold">History</h2>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={() => fetchHistory()}>
            <RefreshCw className="h-4 w-4" />
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onClick={() => setShowClearDialog(true)}
            disabled={history.length === 0}
          >
            <Trash2 className="mr-2 h-4 w-4" /> Clear All
          </Button>
        </div>
      </div>

      <div className="flex-1 overflow-hidden">
        <ScrollArea className="h-[90%]">
          <div className="px-4 pb-12">
            {history.length === 0 ? (
              <div className="flex flex-col items-center text-sm justify-center h-40 text-muted-foreground">
                <p>No history records found</p>
              </div>
            ) : (
              <div className="space-y-3">
                {history
                  .slice()
                  .reverse()
                  .map((item) => (
                    <Card key={item.id} className="overflow-hidden">
                      <CardContent className="p-4 space-y-2.5">
                        <div className="flex flex-col gap-2">
                          <div className="flex justify-between items-start gap-2">
                            <span className="text-xs text-muted-foreground break-words">
                              {new Date(item.timestamp).toLocaleString()}
                            </span>
                            <Button
                              variant="ghost"
                              size="icon-xs"
                              onClick={() => deleteHistoryItem(item.id)}
                              className="shrink-0"
                            >
                              <X className="h-3.5 w-3.5" />
                            </Button>
                          </div>
                          <Badge
                            variant="secondary"
                            className="font-mono text-sm break-words  rounded-md whitespace-normal max-w-full w-fit"
                          >
                            {item.instruction}
                          </Badge>
                        </div>

                        <div className="grid gap-2.5 text-sm">
                          <ScrollArea className="max-h-20 w-full">
                            <div className="bg-muted p-2.5 rounded-md pr-3">
                              <p className="text-xs font-semibold mb-1.5 text-muted-foreground">
                                Original:
                              </p>
                              <p className="whitespace-pre-wrap break-words text-sm leading-relaxed">
                                {item.original_content}
                              </p>
                            </div>
                          </ScrollArea>
                          <ScrollArea className="max-h-28 w-full">
                            <div className="bg-primary/5 p-2.5 rounded-md border border-primary/20 pr-3">
                              <div className="flex items-center justify-between mb-1.5">
                                <p className="text-xs font-semibold text-primary">
                                  Enriched:
                                </p>
                                <Button
                                  variant="ghost"
                                  size="icon-xs"
                                  onClick={() =>
                                    navigator.clipboard.writeText(
                                      item.enriched_content,
                                    )
                                  }
                                  className="shrink-0"
                                  title="Copy to clipboard"
                                >
                                  <Copy className="h-3.5 w-3.5" />
                                </Button>
                              </div>
                              <p className="whitespace-pre-wrap break-words text-sm leading-relaxed">
                                {item.enriched_content}
                              </p>
                            </div>
                          </ScrollArea>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
              </div>
            )}
          </div>
        </ScrollArea>
      </div>

      <AlertDialog open={showClearDialog} onOpenChange={setShowClearDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This will permanently delete all history records. This action
              cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleClearAll}>
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
