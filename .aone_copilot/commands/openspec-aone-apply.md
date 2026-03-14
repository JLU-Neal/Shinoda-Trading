---
name: /openspec-aone-apply
id: openspec-aone-apply
category: OpenSpec
description: Implement an approved OpenSpec change and keep tasks in sync.
---
<!-- OPENSPEC-AONE:START -->
**LANGUAGE**
所有生成的文档，必须使用中文。

**Guardrails**
- 优先选择直接、最小化的实现，仅在请求或明确需要时才增加复杂性。
- 保持变更严格限制在请求的结果范围内。
- 如果需要额外的OpenSpec约定或澄清，请参考 `.aone_copilot/openspec/AGENTS.md`（位于 `openspec/`目录内——如果看不到，运行 `ls openspec` 或 `openspec-aone update`）。

**Steps**
将这些步骤作为TODOs逐一跟踪并完成。
1. 阅读 `changes/<id>/proposal.md`、`design.md`（如果存在）和 `tasks.md` 以确认范围和验收标准。
2. 按顺序处理任务，保持编辑最小化且专注于请求的变更。
3. 在更新状态前确认完成——确保 `tasks.md` 中的每个项目都已完成。
4. 在所有工作完成后更新检查清单，使每个任务标记为 `- [x]` 并反映实际情况。
5. 当需要额外上下文时，参考 `openspec-aone list` 或 `openspec-aone show <item>`。

**Reference**
- 如果在实施过程中需要来自提案的额外上下文，请使用 `openspec-aone show <id> --json --deltas-only`。
<!-- OPENSPEC-AONE:END -->
