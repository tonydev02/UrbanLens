import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { ConnectivityPanel } from "./connectivity-panel";

function renderWithQueryClient(children: ReactNode) {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  return render(<QueryClientProvider client={queryClient}>{children}</QueryClientProvider>);
}

function mockConnectivityResponse(body: unknown) {
  return vi.fn().mockResolvedValue({
    ok: true,
    status: 200,
    headers: new Headers({ "content-type": "application/json" }),
    json: async () => body,
    text: async () => JSON.stringify(body),
  });
}

describe("ConnectivityPanel", () => {
  beforeEach(() => {
    vi.stubEnv("NEXT_PUBLIC_GRAPHQL_URL", "http://localhost:8080/graphql");
    vi.stubGlobal("crypto", { randomUUID: () => "test-request-id" });
  });

  afterEach(() => {
    cleanup();
    vi.unstubAllEnvs();
    vi.unstubAllGlobals();
  });

  it("shows a bounded loading state before the connectivity proof resolves", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(() => new Promise(() => undefined)),
    );

    renderWithQueryClient(<ConnectivityPanel />);

    expect(screen.getByRole("heading", { name: "Checking platform status" })).toBeInTheDocument();
    expect(screen.getByLabelText("Loading API and database connectivity")).toBeInTheDocument();
  });

  it("renders connected API, PostgreSQL, and migration state from GraphQL", async () => {
    vi.stubGlobal(
      "fetch",
      mockConnectivityResponse({
        data: {
          connectivity: {
            service: "urbanlens-api",
            status: "ready",
            databaseReachable: true,
            migrationsApplied: true,
          },
        },
      }),
    );

    renderWithQueryClient(<ConnectivityPanel />);

    expect(await screen.findByRole("heading", { name: "Platform connected" })).toBeInTheDocument();
    expect(screen.getByText("urbanlens-api")).toBeInTheDocument();
    expect(screen.getByText("ready")).toBeInTheDocument();
    expect(screen.getAllByText("Connected")).toHaveLength(2);
    expect(screen.getByText("http://localhost:8080/graphql")).toBeInTheDocument();
  });

  it("renders degraded state returned by the API without treating it as fake success", async () => {
    vi.stubGlobal(
      "fetch",
      mockConnectivityResponse({
        data: {
          connectivity: {
            service: "urbanlens-api",
            status: "not_ready",
            databaseReachable: false,
            migrationsApplied: false,
          },
        },
      }),
    );

    renderWithQueryClient(<ConnectivityPanel />);

    expect(await screen.findByRole("heading", { name: "Platform degraded" })).toBeInTheDocument();
    expect(screen.getByText("not_ready")).toBeInTheDocument();
    expect(screen.getAllByText("Unavailable")).toHaveLength(2);
  });

  it("shows a readable network error and retries from the page", async () => {
    const fetch = vi
      .fn()
      .mockRejectedValueOnce(new Error("Failed to fetch"))
      .mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        text: async () =>
          JSON.stringify({
            data: {
              connectivity: {
                service: "urbanlens-api",
                status: "ready",
                databaseReachable: true,
                migrationsApplied: true,
              },
            },
          }),
        json: async () => ({
          data: {
            connectivity: {
              service: "urbanlens-api",
              status: "ready",
              databaseReachable: true,
              migrationsApplied: true,
            },
          },
        }),
      });

    vi.stubGlobal("fetch", fetch);

    renderWithQueryClient(<ConnectivityPanel />);

    expect(await screen.findByRole("heading", { name: "Connection needs attention" })).toBeInTheDocument();
    expect(screen.getByText("Failed to fetch")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Retry" }));

    await waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
    expect(await screen.findByRole("heading", { name: "Platform connected" })).toBeInTheDocument();
  });
});
