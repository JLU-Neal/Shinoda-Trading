---
name: /openspec-aone-archive
id: openspec-aone-archive
category: OpenSpec
description: Archive a deployed OpenSpec change and update specs.
---
<!-- OPENSPEC-AONE:START -->
**LANGUAGE**
所有生成的文档，必须使用中文。

**Guardrails**
- 优先选择直接、最小化的实现，仅在请求或明确需要时才增加复杂性。
- 保持变更严格限制在请求的结果范围内。
- 如果需要额外的OpenSpec约定或澄清，请参考 `.aone_copilot/openspec/AGENTS.md`（位于 `openspec/`目录内——如果看不到，运行 `ls openspec` 或 `openspec-aone update`）。

**Steps**
1. 确定要归档的变更ID：
   - 如果此提示已包含特定变更ID（例如在由斜杠命令参数填充的 `<ChangeId>` 块内），使用该值并修剪空白。
   - 如果对话松散引用变更（例如通过标题或摘要），运行 `openspec-aone list` 显示可能的ID，分享相关候选项并确认用户打算使用哪一个。
   - 否则，审查对话，运行 `openspec-aone list`，并询问用户要归档哪个变更；等待确认的变更ID后再继续。
   - 如果仍无法识别单个变更ID，停止并告诉用户您暂时无法归档任何内容。
2. 通过运行 `openspec-aone list`（或 `openspec-aone show <id>`）验证变更ID，如果变更丢失、已归档或未准备好归档则停止。
3. 运行 `openspec-aone archive <id> --yes`，使CLI移动变更并应用规范更新而不提示（仅对仅工具工作使用 `--skip-specs`）。
4. 审查命令输出以确认目标规范已更新且变更已移至 `changes/archive/`。
5. 使用 `openspec-aone validate --strict` 验证，如果发现任何问题，使用 `openspec-aone show <id>` 检查。

**Reference**
- 在归档前使用 `openspec-aone list` 确认变更ID。
- 使用 `openspec-aone list --specs` 检查更新的规范，并在交接前解决任何验证问题。
<!-- OPENSPEC-AONE:END -->
