	.text
	.globl main
zero:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq $10, %rax
	movq %rax, -32(%rbp)
L3368:
	movq %rax, -24(%rbp)
	movq -24(%rbp), %r10
	testq %r10, %r10
	jz L3361
	movq $1, -16(%rbp)
	subq -16(%rbp), %rax
	movq %rax, -8(%rbp)
	jmp L3368
L3361:
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3343
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3343
	movq $1, %rax
L3343:
	testq %rax, %rax
L3343:
	jnz L3342
L3340:
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3335
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3335
	movq $1, %rax
L3335:
	testq %rax, %rax
L3335:
	jnz L3334
L3332:
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3327
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3327
	movq $1, %rax
L3327:
	testq %rax, %rax
L3327:
	jnz L3326
L3324:
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3319
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3319
	movq $1, %rax
L3319:
	testq %rax, %rax
L3319:
	jnz L3318
L3316:
	movq $0, %rax
	testq %rax, %rax
	jz L3311
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3311
	movq $1, %rax
L3311:
	testq %rax, %rax
L3311:
	jnz L3310
L3308:
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3303
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3303
	movq $1, %rax
L3303:
	testq %rax, %rax
L3303:
	jnz L3302
L3300:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	call true
	addq $0, %rsp
	testq %rax, %rax
	jnz L3294
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3293
L3294:
	movq $1, %rax
L3293:
	testq %rax, %rax
L3293:
	jnz L3292
L3290:
	call false
	addq $0, %rsp
	testq %rax, %rax
	jnz L3286
	call true
	addq $0, %rsp
	testq %rax, %rax
	jz L3285
L3286:
	movq $1, %rax
L3285:
	testq %rax, %rax
L3285:
	jnz L3284
L3282:
	call true
	addq $0, %rsp
	testq %rax, %rax
	jnz L3278
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3277
L3278:
	movq $1, %rax
L3277:
	testq %rax, %rax
L3277:
	jnz L3276
L3274:
	call false
	addq $0, %rsp
	testq %rax, %rax
	jnz L3270
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3269
L3270:
	movq $1, %rax
L3269:
	testq %rax, %rax
L3269:
	jnz L3268
L3266:
	movq $1, %rax
	testq %rax, %rax
	jnz L3262
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3261
L3262:
	movq $1, %rax
L3261:
	testq %rax, %rax
L3261:
	jnz L3260
L3258:
	call true
	addq $0, %rsp
	testq %rax, %rax
	jnz L3254
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3253
L3254:
	movq $1, %rax
L3253:
	testq %rax, %rax
L3253:
	jnz L3252
L3250:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $65, -16(%rbp)
	call false
	addq $0, %rsp
	testq %rax, %rax
	jz L3242
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3242
	movq $1, %rax
L3242:
	addq %rax, -16(%rbp)
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $64, -8(%rbp)
	call true
	addq $0, %rsp
	testq %rax, %rax
	jnz L3235
	call fail
	addq $0, %rsp
	testq %rax, %rax
	jz L3234
L3235:
	movq $1, %rax
L3234:
	addq %rax, -8(%rbp)
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
	jmp L3234
	jmp L3235
	jmp L3242
	jmp L3242
L3252:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3250
	jmp L3253
	jmp L3254
L3260:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3258
	jmp L3261
	jmp L3262
L3268:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3266
	jmp L3269
	jmp L3270
L3276:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3274
	jmp L3277
	jmp L3278
L3284:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3282
	jmp L3285
	jmp L3286
L3292:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3290
	jmp L3293
	jmp L3294
L3302:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3300
	jmp L3303
	jmp L3303
L3310:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3308
	jmp L3311
	jmp L3311
L3318:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3316
	jmp L3319
	jmp L3319
L3326:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3324
	jmp L3327
	jmp L3327
L3334:
	movq $66, %rdi
	call putchar
	addq $0, %rsp
	jmp L3332
	jmp L3335
	jmp L3335
L3342:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L3340
	jmp L3343
	jmp L3343
fail:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	call any
	movq %rax, -8(%rbp)
	addq $0, %rsp
	call zero
	movq %rax, -16(%rbp)
	addq $0, %rsp
	movq -8(%rbp), %rax
	cqto
	idivq %rax
	movq %rax, -16(%rbp)
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
any:
	ret
false:
	call zero
	addq $0, %rsp
	ret
true:
	call zero
	addq $0, %rsp
	cmpq %rax, $0
	sete %al
	ret
	.data

