	.text
	.globl main
print:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq %rdi, -8(%rbp)
	movq $40, %rdi
	call putchar
	movq -8(%rbp), %rdx
	movq 8(%rdx), %rdi
	movq $0, %rax
	cmpq %rax, %rdi
	setne %dil
	testq %rdi, %rdi
	jnz L450
L447:
	movq -8(%rbp), %rdx
	movq 0(%rdx), %rdi
	call print_int
	movq -8(%rbp), %rdx
	movq 16(%rdx), %r9
	movq $0, %rax
	cmpq %rax, %r9
	setne %r9b
	testq %r9, %r9
	jnz L439
L436:
	movq $41, %rdi
	call putchar
	movq %rbp, %rsp
	popq %rbp
	ret
L439:
	movq -8(%rbp), %rsi
	movq 16(%rsi), %rdi
	call print
	jmp L436
L450:
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdi
	call print
	jmp L447
contient:
	pushq %rbp
	movq %rsp, %rbp
	addq $-96, %rsp
	movq %rdi, %rax
	movq %rsi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -80(%rbp)
	movq %rax, -96(%rbp)
	movq -96(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -88(%rbp)
	movq -88(%rbp), %r10
	movq -80(%rbp), %r11
	cmpq %r10, %r11
	sete %r11b
	movq %r11, -80(%rbp)
	movq -80(%rbp), %r10
	testq %r10, %r10
	jnz L505
	movq -8(%rbp), %r10
	movq %r10, -40(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -64(%rbp)
	movq -64(%rbp), %r10
	movq -40(%rbp), %r11
	cmpq %r10, %r11
	setl %r11b
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	testq %r10, %r10
	jz L493
	movq %rax, -56(%rbp)
	movq -56(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -40(%rbp)
	movq $0, -48(%rbp)
	movq -48(%rbp), %r10
	movq -40(%rbp), %r11
	cmpq %r10, %r11
	setne %r11b
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	testq %r10, %r10
	jz L493
	movq $1, -40(%rbp)
L493:
	movq -40(%rbp), %r10
	testq %r10, %r10
	jnz L492
	movq %rax, -32(%rbp)
	movq -32(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -16(%rbp)
	movq $0, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	cmpq %r10, %r11
	setne %r11b
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L483
	movq $0, %rax
L478:
	movq %rbp, %rsp
	popq %rbp
	ret
L483:
	movq 16(%rax), %rdi
	movq -8(%rbp), %rsi
	call contient
	jmp L478
L492:
	movq 8(%rax), %rdi
	movq -8(%rbp), %rsi
	call contient
	jmp L478
	jmp L493
	jmp L493
L505:
	movq $1, %rax
	jmp L478
make:
	pushq %rbp
	movq %rsp, %rbp
	addq $-80, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, -16(%rbp)
	movq %rdx, -24(%rbp)
	movq $24, %rdi
	call malloc
	movq %rax, -80(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -64(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq -64(%rbp), %r11
	movq %r11, 0(%r10)
	movq -16(%rbp), %r10
	movq %r10, -48(%rbp)
	movq %rax, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	movq %r11, 8(%r10)
	movq -24(%rbp), %r10
	movq %r10, -32(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq -32(%rbp), %r11
	movq %r11, 16(%r10)
	movq %rbp, %rsp
	popq %rbp
	ret
insere:
	pushq %rbp
	movq %rsp, %rbp
	addq $-48, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %rax
	movq %rax, -32(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -48(%rbp)
	movq -48(%rbp), %r10
	movq 0(%r10), %r11
	movq %r11, -40(%rbp)
	movq -40(%rbp), %r10
	movq -32(%rbp), %r11
	cmpq %r10, %r11
	sete %r11b
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	testq %r10, %r10
	jnz L365
	movq %rax, %r9
	movq -8(%rbp), %rdi
	movq 0(%rdi), %rsi
	cmpq %rsi, %r9
	setl %r9b
	testq %r9, %r9
	jnz L359
	movq -8(%rbp), %rdx
	movq 16(%rdx), %rcx
	movq $0, %r8
	cmpq %r8, %rcx
	sete %cl
	testq %rcx, %rcx
	jnz L339
	movq -8(%rbp), %rcx
	movq 16(%rcx), %rdi
	movq %rax, %rsi
	call insere
L329:
	movq $0, %rax
L328:
	movq %rbp, %rsp
	popq %rbp
	ret
L339:
	movq $0, %rsi
	movq $0, %rdx
	movq %rax, %rdi
	call make
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r10
	movq %rax, 16(%r10)
	jmp L329
L359:
	movq -8(%rbp), %rdi
	movq 8(%rdi), %rsi
	movq $0, %rcx
	cmpq %rcx, %rsi
	sete %sil
	testq %rsi, %rsi
	jnz L354
	movq -8(%rbp), %rsi
	movq 8(%rsi), %rdi
	movq %rax, %rsi
	call insere
	jmp L329
L354:
	movq $0, %rsi
	movq $0, %rdx
	movq %rax, %rdi
	call make
	movq -8(%rbp), %r10
	movq %r10, -24(%rbp)
	movq -24(%rbp), %r10
	movq %rax, 8(%r10)
	jmp L329
L365:
	movq $0, %rax
	jmp L328
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-8, %rsp
	movq $1, %rdi
	movq $0, %rsi
	movq $0, %rdx
	call make
	movq %rax, -8(%rbp)
	movq -8(%rbp), %r9
	movq -8(%rbp), %rdi
	movq $17, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $5, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $8, %rsi
	call insere
	movq -8(%rbp), %rdi
	call print
	movq $10, %rdi
	call putchar
	movq -8(%rbp), %rdi
	movq $5, %rsi
	call contient
	testq %rax, %rax
	jz L405
	movq -8(%rbp), %rdi
	movq $0, %rsi
	call contient
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	testq %rax, %rax
	jz L405
	movq $1, %rax
L405:
	testq %rax, %rax
	jz L399
	movq -8(%rbp), %rdi
	movq $17, %rsi
	call contient
	testq %rax, %rax
	jz L399
	movq $1, %rax
L399:
	testq %rax, %rax
	jz L392
	movq -8(%rbp), %rdi
	movq $3, %rsi
	call contient
	movq $0, %r11
	cmpq %rax, %r11
	sete %al
	testq %rax, %rax
	jz L392
	movq $1, %rax
L392:
	testq %rax, %rax
	jnz L391
L385:
	movq -8(%rbp), %rdi
	movq $42, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $1000, %rsi
	call insere
	movq -8(%rbp), %rdi
	movq $0, %rsi
	call insere
	movq -8(%rbp), %rdi
	call print
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L391:
	movq $111, %rdi
	call putchar
	movq $107, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	jmp L385
	jmp L392
	jmp L392
	jmp L399
	jmp L399
	jmp L405
	jmp L405
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %rdi
	movq -16(%rbp), %rax
	cqto
	idivq %rdi
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rsi
	movq -8(%rbp), %rdi
	movq $9, %rax
	cmpq %rax, %rdi
	setg %dil
	testq %rdi, %rdi
	jnz L469
L467:
	movq $48, %rdi
	movq -8(%rbp), %rcx
	movq $10, %rax
	movq -16(%rbp), %rdx
	imulq %rdx, %rax
	subq %rax, %rcx
	addq %rcx, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L469:
	movq -16(%rbp), %rdi
	call print_int
	jmp L467
	.data

