/* eslint-disable @typescript-eslint/no-explicit-any */
// src/api.ts

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:8080";

export type Space = {
  space_id: string;
  space_name: string;
  space_avatar: string;
  active_proposals_count: number;
  proposals_count: number;
};

export async function fetchSpaces(): Promise<Space[]> {
  const response = await fetch(`${API_URL}/spaces`);
  if (!response.ok) {
    throw new Error(`Failed to fetch spaces: ${response.statusText}`);
  }
  const data = await response.json();
  return data as Space[];
}

export type Proposal = {
  id: string;
  title?: string;
  body?: string;
  author?: string;
  quorum: string;
  start?: number;
  end?: number;
  choices?: any;
};

export async function fetchProposals(space_id: string): Promise<Proposal[]> {
  const response = await fetch(`${API_URL}/proposals/${space_id}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch proposals: ${response.statusText}`);
  }
  const data = await response.json();
  return data as Proposal[];
}

export type Recommendation = {
  proposalId: string;
  technicalImpact: any;
  economicConsequences: any;
  governanceAndDecentralization: any;
  advantages: any;
  risks: any;
  recommendation: any;
  createdAt: number;
};

export async function fetchRecommendation(proposalId: string): Promise<Recommendation> {
  const response = await fetch(`${API_URL}/recommendation/${proposalId}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch recommendation: ${response.statusText}`);
  }
  const data = await response.json();
  return data as Recommendation;
}
