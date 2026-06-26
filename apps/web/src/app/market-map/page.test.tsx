import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import MarketMapPage from "./page";

vi.mock("../../components/connectivity-panel", () => ({
  ConnectivityPanel: () => <section aria-label="GraphQL connectivity proof">Connectivity proof</section>,
}));

describe("MarketMapPage", () => {
  it("renders an honest empty market map shell without fake product data", () => {
    const queryClient = new QueryClient();

    render(
      <QueryClientProvider client={queryClient}>
        <MarketMapPage />
      </QueryClientProvider>,
    );

    expect(screen.getByRole("heading", { level: 1, name: "Market map" })).toBeInTheDocument();
    expect(screen.getByRole("heading", { name: "No transaction geography loaded" })).toBeInTheDocument();
    expect(screen.getByLabelText("GraphQL connectivity proof")).toBeInTheDocument();
    expect(screen.queryByText(/median transaction price/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/¥\/m²/i)).not.toBeInTheDocument();
  });
});
