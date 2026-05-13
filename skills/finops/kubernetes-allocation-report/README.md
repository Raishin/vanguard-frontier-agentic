# Kubernetes Allocation Report

A FinOps skill that produces OpenCost-compatible cost allocation tables for Kubernetes namespaces, pods, and workloads using user-supplied cluster topology data and public cloud pricing. No cluster credentials required or accepted.

## Purpose

Generate a namespace-by-namespace, workload-by-workload cost breakdown for a Kubernetes cluster using the node SKU mix, pod resource requests, and runtime hours supplied by the caller. The output attributes cost by namespace, workload, and team label, follows OpenCost allocation semantics, and maps to FOCUS v1.2 billing columns.

## Allowed tools

`Read` `Grep` `Glob` `WebFetch`

## Usage

**Allocation report:** Paste your cluster shape — node SKU mix, namespace list, pod CPU/memory requests, and reporting window in hours. The skill fetches live on-demand node pricing, applies request-based allocation math, and returns a table broken down by namespace, workload, and team label, including idle cost and a FOCUS v1.2 metadata row.

**FOCUS mapping mode:** Provide an OpenCost allocation export and ask for the FOCUS v1.2 column mapping. The skill maps each OpenCost column to its FOCUS equivalent and flags gaps.

## Trust posture

Read-only. No cloud credentials, billing account IDs, or tenant data accepted. No connection to the Kubernetes API server or cloud billing APIs is made. All cluster data must be provided by the caller. Node prices are fetched from public, unauthenticated cloud pricing pages and labeled with source URL and fetch timestamp.

See [SKILL.md](SKILL.md) for the full allocation methodology, required input, and response shape.
