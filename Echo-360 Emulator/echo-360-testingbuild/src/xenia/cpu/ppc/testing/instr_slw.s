test_slw_1:
  #_ REGISTER_IN r4 1
  #_ REGISTER_IN r5 0
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 1
  #_ REGISTER_OUT r4 1
  #_ REGISTER_OUT r5 0

test_slw_1_constant:
  li r4, 1
  li r5, 0
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 1
  #_ REGISTER_OUT r4 1
  #_ REGISTER_OUT r5 0

test_slw_2:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 0
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFF
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 0

test_slw_2_constant:
  li r4, -1
  li r5, 0
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFF
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 0

test_slw_3:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 1
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFE
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 1

test_slw_3_constant:
  li r4, -1
  li r5, 1
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFE
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 1

test_slw_4:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 63
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 63

test_slw_4_constant:
  li r4, -1
  li r5, 63
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 63

test_slw_5:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 64
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFF
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 64

test_slw_5_constant:
  li r4, -1
  li r5, 64
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000FFFFFFFF
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 64

test_slw_6:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 100
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 100

test_slw_6_constant:
  li r4, -1
  li r5, 100
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 100

test_slw_7:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 30
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000c0000000
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 30

test_slw_7_constant:
  li r4, -1
  li r5, 30
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x00000000c0000000
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 30

test_slw_8:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 31
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x0000000080000000
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 31

test_slw_8_constant:
  li r4, -1
  li r5, 31
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0x0000000080000000
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 31

test_slw_9:
  #_ REGISTER_IN r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_IN r5 32
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 32

test_slw_9_constant:
  li r4, -1
  li r5, 32
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 0
  #_ REGISTER_OUT r4 0xFFFFFFFFFFFFFFFF
  #_ REGISTER_OUT r5 32

test_slw_10:
  #_ REGISTER_IN r4 99
  #_ REGISTER_IN r5 1
  cntlzw r5, r5
  subi r5, r5, 28
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 792
  #_ REGISTER_OUT r4 99
  #_ REGISTER_OUT r5 3

test_slw_10_constant:
  #_ REGISTER_IN r4 99
  li r5, 1
  cntlzw r5, r5
  subi r5, r5, 28
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 792
  #_ REGISTER_OUT r4 99
  #_ REGISTER_OUT r5 3

test_slw_11:
  #_ REGISTER_IN r4 99
  #_ REGISTER_IN r5 3
  li r5, 3
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 792
  #_ REGISTER_OUT r4 99
  #_ REGISTER_OUT r5 3

test_slw_11_constant:
  #_ REGISTER_IN r4 99
  li r5, 3
  slw r3, r4, r5
  blr
  #_ REGISTER_OUT r3 792
  #_ REGISTER_OUT r4 99
  #_ REGISTER_OUT r5 3
