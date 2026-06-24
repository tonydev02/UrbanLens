import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import HomePage from "./page";

describe("HomePage", () => {
  it("identifies the workspace as a foundation without claiming product data", () => {
    render(<HomePage />);

    expect(
      screen.getByRole("heading", { level: 1, name: "UrbanLens workspace foundation" }),
    ).toBeInTheDocument();
    expect(
      screen.getByText(/Analyst routes and public-data connectivity arrive in later/i),
    ).toBeInTheDocument();
  });
});
