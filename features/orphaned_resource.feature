Feature: orphaned resource

    Given file "1.md" with content:
      """
      # Title

      <img src="non-existing.png">
      <img src="non-existing.png" />
      ![broken image](non-existing.png)
      """
