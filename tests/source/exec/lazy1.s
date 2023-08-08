	.text
	.globl main
any:
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	call true
	testq %rax, %rax
	jz L3473
	call true
	testq %rax, %rax
	jz L3473
	movq $1, %rax
L3473:
	testq %rax, %rax
	jnz L3472
L3470:
	call false
	testq %rax, %rax
	jz L3465
	call true
	testq %rax, %rax
	jz L3465
	movq $1, %rax
L3465:
	testq %rax, %rax
	jnz L3464
L3462:
	call true
	testq %rax, %rax
	jz L3457
	call false
	testq %rax, %rax
	jz L3457
	movq $1, %rax
L3457:
	testq %rax, %rax
	jnz L3456
L3454:
	call false
	testq %rax, %rax
	jz L3449
	call false
	testq %rax, %rax
	jz L3449
	movq $1, %rax
L3449:
	testq %rax, %rax
	jnz L3448
L3446:
	movq $0, %rax
	testq %rax, %rax
	jz L3441
	call fail
	testq %rax, %rax
	jz L3441
	movq $1, %rax
L3441:
	testq %rax, %rax
	jnz L3440
L3438:
	call false
	testq %rax, %rax
	jz L3433
	call fail
	testq %rax, %rax
	jz L3433
	movq $1, %rax
L3433:
	testq %rax, %rax
	jnz L3432
L3430:
	movq $10, %rdi
	call putchar
	call true
	testq %rax, %rax
	jnz L3424
	call true
	testq %rax, %rax
	jz L3423
L3424:
	movq $1, %rax
L3423:
	testq %rax, %rax
	jnz L3422
L3420:
	call false
	testq %rax, %rax
	jnz L3416
	call true
	testq %rax, %rax
	jz L3415
L3416:
	movq $1, %rax
L3415:
	testq %rax, %rax
	jnz L3414
L3412:
	call true
	testq %rax, %rax
	jnz L3408
	call false
	testq %rax, %rax
	jz L3407
L3408:
	movq $1, %rax
L3407:
	testq %rax, %rax
	jnz L3406
L3404:
	call false
	testq %rax, %rax
	jnz L3400
	call false
	testq %rax, %rax
	jz L3399
L3400:
	movq $1, %rax
L3399:
	testq %rax, %rax
	jnz L3398
L3396:
	movq $1, %rax
	testq %rax, %rax
	jnz L3392
	call fail
	testq %rax, %rax
	jz L3391
L3392:
	movq $1, %rax
L3391:
	testq %rax, %rax
	jnz L3390
L3388:
	call true
	testq %rax, %rax
	jnz L3384
	call fail
	testq %rax, %rax
	jz L3383
L3384:
	movq $1, %rax
L3383:
	testq %rax, %rax
	jnz L3382
L3380:
	movq $10, %rdi
	call putchar
	movq $65, -16(%rbp)
	call false
	testq %rax, %rax
	jz L3372
	call fail
	testq %rax, %rax
	jz L3372
	movq $1, %rax
L3372:
	addq %rax, -16(%rbp)
	movq -16(%rbp), %rdi
	call putchar
	movq $64, -8(%rbp)
	call true
	testq %rax, %rax
	jnz L3365
	call fail
	testq %rax, %rax
	jz L3364
L3365:
	movq $1, %rax
L3364:
	addq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	jmp L3364
	jmp L3365
	jmp L3372
	jmp L3372
L3382:
	movq $65, %rdi
	call putchar
	jmp L3380
	jmp L3383
	jmp L3384
L3390:
	movq $65, %rdi
	call putchar
	jmp L3388
	jmp L3391
	jmp L3392
L3398:
	movq $66, %rdi
	call putchar
	jmp L3396
	jmp L3399
	jmp L3400
L3406:
	movq $65, %rdi
	call putchar
	jmp L3404
	jmp L3407
	jmp L3408
L3414:
	movq $65, %rdi
	call putchar
	jmp L3412
	jmp L3415
	jmp L3416
L3422:
	movq $65, %rdi
	call putchar
	jmp L3420
	jmp L3423
	jmp L3424
L3432:
	movq $66, %rdi
	call putchar
	jmp L3430
	jmp L3433
	jmp L3433
L3440:
	movq $66, %rdi
	call putchar
	jmp L3438
	jmp L3441
	jmp L3441
L3448:
	movq $66, %rdi
	call putchar
	jmp L3446
	jmp L3449
	jmp L3449
L3456:
	movq $66, %rdi
	call putchar
	jmp L3454
	jmp L3457
	jmp L3457
L3464:
	movq $66, %rdi
	call putchar
	jmp L3462
	jmp L3465
	jmp L3465
L3472:
	movq $65, %rdi
	call putchar
	jmp L3470
	jmp L3473
	jmp L3473
zero:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq $10, %rax
	movq %rax, -32(%rbp)
L3487:
	movq %rax, -24(%rbp)
	movq -24(%rbp), %r10
	testq %r10, %r10
	jz L3480
	movq $1, -16(%rbp)
	subq -16(%rbp), %rax
	movq %rax, -8(%rbp)
	jmp L3487
L3480:
	movq %rbp, %rsp
	popq %rbp
	ret
false:
	call zero
	ret
fail:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	call any
	movq %rax, -8(%rbp)
	call zero
	movq %rax, -16(%rbp)
	movq -8(%rbp), %rax
	cqto
	idivq -16(%rbp)
	movq %rax, -8(%rbp)
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
true:
	call zero
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	ret
	.data

