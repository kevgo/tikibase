Feature: orphaned resource

    Given file "1.md" with content:
      """
      # Title

      <img src="existing.png">
      <img src="existing.png" />
      ![valid image](existing.png)

      <img src="non-existing.png">
      <img src="non-existing.png" />
      ![broken image](non-existing.png)
      """
