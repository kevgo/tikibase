Feature: recognize/fix broken images

  Background:
    Given file "1.md" with content:
      """
      # Title

      <img src="existing.png">
      <img src="existing.png" />
      ![valid image](existing.png)

      <img src="non-existing.png">
      <img src="non-existing.png" />
      ![broken image](non-existing.png)

      <img src="existing.png">
      <img src="existing.png" />
      ![valid image](existing.png)
      """
    And binary file "existing.png"

  Scenario: this
    When checking
    Then it prints:
      """
      1.md:3  broken image "non-existing.png"
      1.md:4  broken image "non-existing.png"
      1.md:5  broken image "non-existing.png"
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      1.md:3  broken image "non-existing.png"
      1.md:4  broken image "non-existing.png"
      1.md:5  broken image "non-existing.png"
      """
