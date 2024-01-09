test_vupklsb_1:
  # {-8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7}
  #_ REGISTER_IN v3 [F8F9FAFB, FCFDFEFF, 00010203, 04050607]
  vupklsb v3, v3
  blr
  # {0, 1, 2, 3, 4, 5, 6, 7}
  #_ REGISTER_OUT v3 [00000001, 00020003, 00040005, 00060007]

test_vupklsb_2:
  # {0, 255, 255, 0, 0, 0, 255, 0, 255, 0, 0, 255, 255, 255, 0, 255}
  #_ REGISTER_IN v3 [00FFFF00, 0000FF00, FF0000FF, FFFF00FF]
  vupklsb v3, v3
  blr
  # {65535, 0, 0, 65535, 65535, 65535, 0, 65535}
  #_ REGISTER_OUT v3 [FFFF0000, 0000FFFF, FFFFFFFF, 0000FFFF]
