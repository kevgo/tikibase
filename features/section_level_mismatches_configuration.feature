Feature: recognize sections with heading levels different from the configured ones

  Background:
    Given file "tikibase.json" with content:
      """
      {
        "sections": [
          "### alpha",
          "### beta"
        ]
      }
      """
    Given file "1.md" with content:
      """
      # One
      ### alpha
      [Two](2.md)
      """
    And file "2.md" with content:
      """
      # Two
      ##### alpha
      [One](1.md)
      """

  Scenario: check
    When checking
    Then it prints:
      """
      2.md:2  heading level (<h5>) of "##### alpha" differs from configured level (<h3>)
      """
    And the exit code is 1

  Scenario: fix
    When fixing
    Then it prints:
      """
      2.md:2  normalized section "##### alpha" from <h5> to <h3>
      """
    And file "2.md" should contain:
      """
      # Two
      ### alpha
      [One](1.md)
      """

  Scenario: pitstop
    When doing a pitstop
    Then it prints:
      """
      2.md:2  normalized section "##### alpha" from <h5> to <h3>
      """
    And file "2.md" should contain:
      """
      # Two
      ### alpha
      [One](1.md)
      """
    And the exit code is 0
