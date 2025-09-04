Feature: "search" command

  Scenario: no search terms provided
    When searching for nothing
    Then the exit code is 0
    And it prints:
      """
      No search terms provided
      """

  Scenario: single search term found
    Given file "doc1.md" with content:
      """
      # Document One
      This is a test document.
      It contains some example content.
      """
    Given file "doc2.md" with content:
      """
      # Document Two
      This is another document.
      No matching content here.
      """
    When searching for "test"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This is a test document.

      """

  Scenario: single search term not found
    Given file "doc1.md" with content:
      """
      # Document One
      This is a sample document.
      """
    When searching for "missing"
    Then the exit code is 0
    And it prints nothing

  Scenario: multiple search terms all found in same document
    Given file "doc1.md" with content:
      """
      # Document One
      This is a test document.
      It contains sample content for testing.
      """
    When searching for "test" and "sample"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This is a test document.
        3: It contains sample content for testing.

      """

  Scenario: multiple search terms found in different lines
    Given file "doc1.md" with content:
      """
      # Document One
      This line contains the first term.
      This line contains the second term.
      This line has neither.
      """
    When searching for "first" and "second"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This line contains the first term.
        3: This line contains the second term.

      """

  Scenario: search terms found in multiple documents
    Given file "doc1.md" with content:
      """
      # Document One
      This is a test document.
      """
    Given file "doc2.md" with content:
      """
      # Document Two
      Another test file here.
      """
    When searching for "test"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This is a test document.

      doc2.md
        2: Another test file here.

      """

  Scenario: case insensitive search
    Given file "doc1.md" with content:
      """
      # Document One
      This contains TEST in uppercase.
      This contains Test in mixed case.
      This contains test in lowercase.
      """
    When searching for "test"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This contains TEST in uppercase.
        3: This contains Test in mixed case.
        4: This contains test in lowercase.

      """

  Scenario: partial match in search terms
    Given file "doc1.md" with content:
      """
      # Document One
      This document contains testing information.
      """
    When searching for "test"
    Then the exit code is 0
    And it prints:
      """
      doc1.md
        2: This document contains testing information.

      """

  Scenario: multiple search terms with one missing
    Given file "doc1.md" with content:
      """
      # Document One
      This is a test document.
      """
    When searching for "test" and "missing"
    Then the exit code is 0
    And it prints nothing

