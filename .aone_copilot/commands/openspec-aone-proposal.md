---
name: /openspec-aone-proposal
id: openspec-aone-proposal
category: OpenSpec
description: Scaffold a new OpenSpec change and validate strictly.
---
<!-- OPENSPEC-AONE:START -->
**LANGUAGE**
所有生成的文档，必须使用中文。

**Guardrails**
- 优先选择直接、最小化的实现，仅在请求或明确需要时才增加复杂性。
- 保持变更严格限制在请求的结果范围内。
- 如果需要额外的OpenSpec约定或澄清，请参考 `.aone_copilot/openspec/AGENTS.md`（位于 `openspec/`目录内——如果看不到，运行 `ls openspec` 或 `openspec-aone update`）。
- 在编辑文件之前，识别任何模糊或不明确的细节，并询问必要的后续问题。

**Steps**
1. 审查 `.aone_copilot/openspec/project.md`，运行 `openspec-aone list` 和 `openspec-aone list --specs`，并检查相关代码或文档（例如通过 `rg`/`ls`）以使提案基于当前行为；注意需要澄清的任何空白。
2. 选择唯一的动词引导的 `change-id`，并在 `.aone_copilot/openspec/changes/<id>/` 下创建 `proposal.md`、`tasks.md` 和 `design.md`（需要时）的脚手架。
3. 将变更映射到具体功能或需求，将多范围工作分解为具有清晰关系和排序的不同规范增量。
4. 当解决方案跨越多个系统、引入新模式或在提交规范前需要权衡讨论时，在 `design.md` 中捕获架构推理。
5. 在 `changes/<id>/specs/<capability>/spec.md` 中（每个功能一个文件夹）使用 `## ADDED|MODIFIED|REMOVED Requirements` 起草规范增量，每个需求至少包含一个 `#### Scenario:`，并在相关时交叉引用相关功能。
6. 将 `tasks.md` 起草为小的、可验证的工作项目的有序列表，这些项目提供用户可见的进度，包括验证（测试、工具），并突出显示依赖关系或可并行的工作。
7. 使用 `openspec-aone validate <id> --strict` 验证并在共享提案前解决每个问题。

**Reference**
- 当验证失败时，使用 `openspec-aone show <id> --json --deltas-only` 或 `openspec-aone show <spec> --type spec` 检查详细信息。
- 在编写新需求之前，使用 `rg -n "Requirement:|Scenario:" .aone_copilot/openspec/specs` 搜索现有需求。
- 使用 `rg <keyword>`、`ls` 或直接文件读取探索代码库，使提案与当前实现现实一致。
<!-- OPENSPEC-AONE:END -->
