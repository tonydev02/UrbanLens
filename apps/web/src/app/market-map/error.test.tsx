import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import MarketMapError from "./error";

describe("MarketMapError", () => {
  it("keeps the route failure readable and retryable", () => {
    const reset = vi.fn();

    render(<MarketMapError error={new Error("Route failed")} reset={reset} />);

    expect(screen.getByRole("heading", { name: "Market map could not load" })).toBeInTheDocument();
    expect(screen.getByText("Route failed")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Retry" }));

    expect(reset).toHaveBeenCalledOnce();
  });
});
