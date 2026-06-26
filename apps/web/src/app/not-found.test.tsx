import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import NotFound from "./not-found";

describe("NotFound", () => {
  it("routes users back to the market map workspace", () => {
    render(<NotFound />);

    expect(screen.getByRole("heading", { name: "Workspace route not found" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Open Market Map" })).toHaveAttribute("href", "/market-map");
  });
});
